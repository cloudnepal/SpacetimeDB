namespace SpacetimeDB.BSATN;

public interface ITypeRegistrar
{
    AlgebraicType.Ref RegisterType<T>(Func<AlgebraicType.Ref, AlgebraicType> type);
}

[SpacetimeDB.Type]
public partial struct AggregateElement(string? name, AlgebraicType algebraicType)
{
    public string? Name = name;
    public AlgebraicType AlgebraicType = algebraicType;
}

[SpacetimeDB.Type]
public partial struct MapElement(AlgebraicType key, AlgebraicType value)
{
    public AlgebraicType Key = key;
    public AlgebraicType Value = value;
}

[SpacetimeDB.Type]
public partial record BuiltinType
    : SpacetimeDB.TaggedEnum<(
        Unit Bool,
        Unit I8,
        Unit U8,
        Unit I16,
        Unit U16,
        Unit I32,
        Unit U32,
        Unit I64,
        Unit U64,
        Unit I128,
        Unit U128,
        Unit F32,
        Unit F64,
        Unit String,
        AlgebraicType Array,
        MapElement Map
    )> { }

[SpacetimeDB.Type]
public partial record AlgebraicType
    : SpacetimeDB.TaggedEnum<(
        AggregateElement[] Sum,
        AggregateElement[] Product,
        BuiltinType Builtin,
        int Ref
    )>
{
    public static implicit operator AlgebraicType(BuiltinType builtin) => new Builtin(builtin);

    public static readonly AlgebraicType Unit = new Product([]);

    // Special AlgebraicType that can be recognised by the SpacetimeDB `generate` CLI as an Option<T>.
    internal static AlgebraicType MakeOption(AlgebraicType someType) =>
        new Sum([new("some", someType), new("none", Unit)]);
}
