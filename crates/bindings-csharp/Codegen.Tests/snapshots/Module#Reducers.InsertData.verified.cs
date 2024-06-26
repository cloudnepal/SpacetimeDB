﻿//HintName: Reducers.InsertData.cs

// <auto-generated />
#nullable enable

partial class Reducers
{
    public static SpacetimeDB.Runtime.ScheduleToken ScheduleInsertData(
        DateTimeOffset time,
        PublicTable data
    )
    {
        using var stream = new MemoryStream();
        using var writer = new BinaryWriter(stream);
        new PublicTable.BSATN().Write(writer, data);
        return new(nameof(InsertData), stream.ToArray(), time);
    }
} // Reducers
