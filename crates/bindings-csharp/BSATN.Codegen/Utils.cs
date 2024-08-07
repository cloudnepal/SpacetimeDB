namespace SpacetimeDB.Codegen;

using System.Collections;
using System.Collections.Immutable;
using System.Text;
using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp.Syntax;
using static System.Collections.StructuralComparisons;

public static class Utils
{
    // Even `ImmutableArray<T>` is not deeply equatable, which makes it a common
    // pain point for source generators as they must use only cacheable types.
    // As a result, everyone builds their own `EquatableArray<T>` type.
    public readonly record struct EquatableArray<T>(ImmutableArray<T> Array) : IEnumerable<T>
        where T : IEquatable<T>
    {
        public int Length => Array.Length;
        public T this[int index] => Array[index];

        public bool Equals(EquatableArray<T> other) => Array.SequenceEqual(other.Array);

        public override int GetHashCode() => StructuralEqualityComparer.GetHashCode(Array);

        public IEnumerator<T> GetEnumerator() => ((IEnumerable<T>)Array).GetEnumerator();

        IEnumerator IEnumerable.GetEnumerator() => ((IEnumerable)Array).GetEnumerator();
    }

    private static readonly SymbolDisplayFormat SymbolFormat = SymbolDisplayFormat
        .FullyQualifiedFormat.WithGlobalNamespaceStyle(SymbolDisplayGlobalNamespaceStyle.Omitted)
        .AddMemberOptions(SymbolDisplayMemberOptions.IncludeContainingType)
        .AddMiscellaneousOptions(
            SymbolDisplayMiscellaneousOptions.IncludeNullableReferenceTypeModifier
        );

    public static string SymbolToName(ISymbol symbol)
    {
        return symbol.ToDisplayString(SymbolFormat);
    }

    public static void RegisterSourceOutputs(
        this IncrementalValuesProvider<KeyValuePair<string, string>> methods,
        IncrementalGeneratorInitializationContext context
    )
    {
        context.RegisterSourceOutput(
            methods,
            (context, method) =>
            {
                context.AddSource(
                    $"{string.Join("_", method.Key.Split(Path.GetInvalidFileNameChars()))}.cs",
                    $"""
                    // <auto-generated />
                    #nullable enable

                    {method.Value}
                    """
                );
            }
        );
    }

    public static string MakeRwTypeParam(string typeParam) => typeParam + "RW";

    public static string GetTypeInfo(ITypeSymbol type)
    {
        // We need to distinguish handle nullable reference types specially:
        // compiler expands something like `int?` to `System.Nullable<int>` with the nullable annotation set to `Annotated`
        // while something like `string?` is expanded to `string` with the nullable annotation set to `Annotated`...
        // Beautiful design requires beautiful hacks.
        if (
            type.NullableAnnotation == NullableAnnotation.Annotated
            && type.OriginalDefinition.SpecialType != SpecialType.System_Nullable_T
        )
        {
            // If we're here, then this is a nullable reference type like `string?` and the original definition is `string`.
            type = type.WithNullableAnnotation(NullableAnnotation.None);
            return $"SpacetimeDB.BSATN.RefOption<{type}, {GetTypeInfo(type)}>";
        }
        return type switch
        {
            ITypeParameterSymbol typeParameter => MakeRwTypeParam(typeParameter.Name),
            INamedTypeSymbol namedType
                => type.SpecialType switch
                {
                    SpecialType.System_Boolean => "SpacetimeDB.BSATN.Bool",
                    SpecialType.System_SByte => "SpacetimeDB.BSATN.I8",
                    SpecialType.System_Byte => "SpacetimeDB.BSATN.U8",
                    SpecialType.System_Int16 => "SpacetimeDB.BSATN.I16",
                    SpecialType.System_UInt16 => "SpacetimeDB.BSATN.U16",
                    SpecialType.System_Int32 => "SpacetimeDB.BSATN.I32",
                    SpecialType.System_UInt32 => "SpacetimeDB.BSATN.U32",
                    SpecialType.System_Int64 => "SpacetimeDB.BSATN.I64",
                    SpecialType.System_UInt64 => "SpacetimeDB.BSATN.U64",
                    SpecialType.System_Single => "SpacetimeDB.BSATN.F32",
                    SpecialType.System_Double => "SpacetimeDB.BSATN.F64",
                    SpecialType.System_String => "SpacetimeDB.BSATN.String",
                    SpecialType.None => GetTypeInfoForNamedType(namedType),
                    _
                        => throw new InvalidOperationException(
                            $"Unsupported special type {type} ({type.SpecialType})"
                        )
                },
            IArrayTypeSymbol { ElementType: var elementType }
                => elementType.SpecialType == SpecialType.System_Byte
                    ? "SpacetimeDB.BSATN.ByteArray"
                    : $"SpacetimeDB.BSATN.Array<{elementType}, {GetTypeInfo(elementType)}>",
            _ => throw new InvalidOperationException($"Unsupported type {type}")
        };

        static string GetTypeInfoForNamedType(INamedTypeSymbol type)
        {
            if (type.TypeKind == Microsoft.CodeAnalysis.TypeKind.Error)
            {
                throw new InvalidOperationException($"Could not resolve type {type}");
            }
            if (type.TypeKind == Microsoft.CodeAnalysis.TypeKind.Enum)
            {
                if (
                    !type.GetAttributes()
                        .Any(a => a.AttributeClass?.ToString() == "SpacetimeDB.TypeAttribute")
                )
                {
                    throw new InvalidOperationException(
                        $"Enum {type} does not have a [SpacetimeDB.Type] attribute"
                    );
                }
                return $"SpacetimeDB.BSATN.Enum<{SymbolToName(type)}>";
            }
            var result = type.OriginalDefinition.ToString() switch
            {
                // (U)Int128 are not treated by C# as regular primitives, so we need to match them by type name.
                "System.Int128" => "SpacetimeDB.BSATN.I128",
                "System.UInt128" => "SpacetimeDB.BSATN.U128",
                "System.Collections.Generic.List<T>" => $"SpacetimeDB.BSATN.List",
                "System.Collections.Generic.Dictionary<TKey, TValue>"
                    => $"SpacetimeDB.BSATN.Dictionary",
                // If we're here, then this is nullable *value* type like `int?`.
                "System.Nullable<T>" => $"SpacetimeDB.BSATN.ValueOption",
                var name when name.StartsWith("System.")
                    => throw new InvalidOperationException($"Unsupported system type {name}"),
                _ => $"{SymbolToName(type)}.BSATN"
            };
            if (type.IsGenericType)
            {
                result =
                    $"{result}<{string.Join(", ", type.TypeArguments.Select(SymbolToName).Concat(type.TypeArguments.Select(GetTypeInfo)))}>";
            }
            return result;
        }
    }

    public static IEnumerable<IFieldSymbol> GetFields(
        TypeDeclarationSyntax typeSyntax,
        INamedTypeSymbol type
    )
    {
        // Note: we could use naively use `type.GetMembers()` to get all fields of the type,
        // but some users add their own fields in extra partial declarations like this:
        //
        // ```csharp
        // [SpacetimeDB.Type]
        // partial class MyType
        // {
        //     public int TableField;
        // }
        //
        // partial class MyType
        // {
        //     public int ExtraField;
        // }
        // ```
        //
        // In this scenario, only fields declared inside the declaration with the `[SpacetimeDB.Type]` attribute
        // should be considered as BSATN fields, and others are expected to be ignored.
        //
        // To achieve this, we need to walk over the annotated type syntax node, collect the field names,
        // and look up the resolved field symbols only for those fields.
        return typeSyntax
            .Members.OfType<FieldDeclarationSyntax>()
            .SelectMany(f => f.Declaration.Variables)
            .SelectMany(v => type.GetMembers(v.Identifier.Text))
            .OfType<IFieldSymbol>()
            .Where(f => !f.IsStatic);
    }

    // Borrowed & modified code for generating in-place extensions for partial structs/classes/etc. Source:
    // https://andrewlock.net/creating-a-source-generator-part-5-finding-a-type-declarations-namespace-and-type-hierarchy/

    public readonly record struct Scope
    {
        // Reversed list of typescopes, from innermost to outermost.
        private readonly EquatableArray<TypeScope> typeScopes;

        // Reversed list of namespaces, from innermost to outermost.
        private readonly EquatableArray<string> namespaces;

        public Scope(MemberDeclarationSyntax? node)
        {
            var typeScopes_ = ImmutableArray.CreateBuilder<TypeScope>();
            // Keep looping while we're in a supported nested type
            while (node is TypeDeclarationSyntax type)
            {
                // Record the parent type keyword (class/struct etc), name, and constraints
                typeScopes_.Add(
                    new TypeScope(
                        Keyword: type.Keyword.ValueText,
                        Name: type.Identifier.ToString() + type.TypeParameterList,
                        Constraints: type.ConstraintClauses.ToString()
                    )
                ); // set the child link (null initially)

                // Move to the next outer type
                node = type.Parent as MemberDeclarationSyntax;
            }
            typeScopes = new(typeScopes_.ToImmutable());

            // We've now reached the outermost type, so we can determine the namespace
            var namespaces_ = ImmutableArray.CreateBuilder<string>();
            while (node is BaseNamespaceDeclarationSyntax ns)
            {
                namespaces_.Add(ns.Name.ToString());
                node = node.Parent as MemberDeclarationSyntax;
            }
            namespaces = new(namespaces_.ToImmutable());
        }

        public readonly record struct TypeScope(string Keyword, string Name, string Constraints);

        public string GenerateExtensions(
            string contents,
            string? interface_ = null,
            string? extraAttrs = null
        )
        {
            var sb = new StringBuilder();

            // Join all namespaces into a single namespace statement, starting with the outermost.
            if (namespaces.Length > 0)
            {
                sb.Append("namespace ");
                var first = true;
                foreach (var ns in namespaces.Reverse())
                {
                    if (!first)
                    {
                        sb.Append('.');
                    }
                    first = false;
                    sb.Append(ns);
                }
                sb.AppendLine(" {");
            }

            // Loop through the full parent type hiearchy, starting with the outermost.
            foreach (var (i, typeScope) in typeScopes.Select((ts, i) => (i, ts)).Reverse())
            {
                if (i == 0 && extraAttrs is not null)
                {
                    sb.AppendLine(extraAttrs);
                }

                sb.Append("partial ")
                    .Append(typeScope.Keyword) // e.g. class/struct/record
                    .Append(' ')
                    .Append(typeScope.Name) // e.g. Outer/Generic<T>
                    .Append(' ');

                if (i == 0 && interface_ is not null)
                {
                    sb.Append(" : ").Append(interface_);
                }

                sb.Append(typeScope.Constraints).AppendLine(" {");
            }

            sb.AppendLine();
            sb.Append(contents);
            sb.AppendLine();

            // We need to "close" each of the parent types, so write
            // the required number of '}'
            foreach (var typeScope in typeScopes)
            {
                sb.Append("} // ").AppendLine(typeScope.Name);
            }

            // Close the namespace, if we had one
            if (namespaces.Length > 0)
            {
                sb.AppendLine("} // namespace");
            }

            return sb.ToString();
        }
    }
}
