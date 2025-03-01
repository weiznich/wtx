create_enum! {
  /// Type
  #[derive(Clone, Copy, Debug, Eq, PartialEq)]
  pub enum Ty<u32> {
    Bool = (16, "bool"),
    Bytea = (17, "bytea"),
    Char = (18, "char"),
    Name = (19, "name"),
    Int8 = (20, "int8"),
    Int2 = (21, "int2"),
    Int2Vector = (22, "int2vector"),
    Int4 = (23, "int4"),
    Regproc = (24, "regproc"),
    Text = (25, "text"),
    Oid = (26, "oid"),
    Tid = (27, "tid"),
    Xid = (28, "xid"),
    Cid = (29, "cid"),
    OidVector = (30, "oidvector"),
    PgDdlCommand = (32, "pg_ddl_command"),
    Json = (114, "json"),
    Xml = (142, "xml"),
    XmlArray = (143, "_xml"),
    PgNodeTree = (194, "pg_node_tree"),
    JsonArray = (199, "_json"),
    TableAmHandler = (269, "table_am_handler"),
    Xid8Array = (271, "_xid8"),
    IndexAmHandler = (325, "index_am_handler"),
    Point = (600, "point"),
    Lseg = (601, "lseg"),
    Path = (602, "path"),
    Box = (603, "box"),
    Polygon = (604, "polygon"),
    Line = (628, "line"),
    LineArray = (629, "_line"),
    Cidr = (650, "cidr"),
    CidrArray = (651, "_cidr"),
    Float4 = (700, "float4"),
    Float8 = (701, "float8"),
    Unknown = (705, "unknown"),
    Circle = (718, "circle"),
    CircleArray = (719, "_circle"),
    Macaddr8 = (774, "macaddr8"),
    Macaddr8Array = (775, "_macaddr8"),
    Money = (790, "money"),
    MoneyArray = (791, "_money"),
    Macaddr = (829, "macaddr"),
    Inet = (869, "inet"),
    BoolArray = (1000, "_bool"),
    ByteaArray = (1001, "_bytea"),
    CharArray = (1002, "_char"),
    NameArray = (1003, "_name"),
    Int2Array = (1005, "_int2"),
    Int2VectorArray = (1006, "_int2vector"),
    Int4Array = (1007, "_int4"),
    RegprocArray = (1008, "_regproc"),
    TextArray = (1009, "_text"),
    TidArray = (1010, "_tid"),
    XidArray = (1011, "_xid"),
    CidArray = (1012, "_cid"),
    OidVectorArray = (1013, "_oidvector"),
    BpcharArray = (1014, "_bpchar"),
    VarcharArray = (1015, "_varchar"),
    Int8Array = (1016, "_int8"),
    PointArray = (1017, "_point"),
    LsegArray = (1018, "_lseg"),
    PathArray = (1019, "_path"),
    BoxArray = (1020, "_box"),
    Float4Array = (1021, "_float4"),
    Float8Array = (1022, "_float8"),
    PolygonArray = (1027, "_polygon"),
    OidArray = (1028, "_oid"),
    Aclitem = (1033, "aclitem"),
    AclitemArray = (1034, "_aclitem"),
    MacaddrArray = (1040, "_macaddr"),
    InetArray = (1041, "_inet"),
    Bpchar = (1042, "bpchar"),
    Varchar = (1043, "varchar"),
    Date = (1082, "date"),
    Time = (1083, "time"),
    Timestamp = (1114, "timestamp"),
    TimestampArray = (1115, "_timestamp"),
    DateArray = (1182, "_date"),
    TimeArray = (1183, "_time"),
    Timestamptz = (1184, "timestamptz"),
    TimestamptzArray = (1185, "_timestamptz"),
    Interval = (1186, "interval"),
    IntervalArray = (1187, "_interval"),
    NumericArray = (1231, "_numeric"),
    CstringArray = (1263, "_cstring"),
    Timetz = (1266, "timetz"),
    TimetzArray = (1270, "_timetz"),
    Bit = (1560, "bit"),
    BitArray = (1561, "_bit"),
    Varbit = (1562, "varbit"),
    VarbitArray = (1563, "_varbit"),
    Numeric = (1700, "numeric"),
    Refcursor = (1790, "refcursor"),
    RefcursorArray = (2201, "_refcursor"),
    Regprocedure = (2202, "regprocedure"),
    Regoper = (2203, "regoper"),
    Regoperator = (2204, "regoperator"),
    Regclass = (2205, "regclass"),
    Regtype = (2206, "regtype"),
    RegprocedureArray = (2207, "_regprocedure"),
    RegoperArray = (2208, "_regoper"),
    RegoperatorArray = (2209, "_regoperator"),
    RegclassArray = (2210, "_regclass"),
    RegtypeArray = (2211, "_regtype"),
    Record = (2249, "record"),
    Cstring = (2275, "cstring"),
    Any = (2276, "any"),
    Anyarray = (2277, "anyarray"),
    Void = (2278, "void"),
    Trigger = (2279, "trigger"),
    LanguageHandler = (2280, "language_handler"),
    Internal = (2281, "internal"),
    Anyelement = (2283, "anyelement"),
    RecordArray = (2287, "_record"),
    Anynonarray = (2776, "anynonarray"),
    TxidSnapshotArray = (2949, "_txid_snapshot"),
    Uuid = (2950, "uuid"),
    UuidArray = (2951, "_uuid"),
    TxidSnapshot = (2970, "txid_snapshot"),
    FdwHandler = (3115, "fdw_handler"),
    PgLsn = (3220, "pg_lsn"),
    PgLsnArray = (3221, "_pg_lsn"),
    TsmHandler = (3310, "tsm_handler"),
    PgNdistinct = (3361, "pg_ndistinct"),
    PgDependencies = (3402, "pg_dependencies"),
    Anyenum = (3500, "anyenum"),
    TsVector = (3614, "tsvector"),
    Tsquery = (3615, "tsquery"),
    GtsVector = (3642, "gtsvector"),
    TsVectorArray = (3643, "_tsvector"),
    GtsVectorArray = (3644, "_gtsvector"),
    TsqueryArray = (3645, "_tsquery"),
    Regconfig = (3734, "regconfig"),
    RegconfigArray = (3735, "_regconfig"),
    Regdictionary = (3769, "regdictionary"),
    RegdictionaryArray = (3770, "_regdictionary"),
    Jsonb = (3802, "jsonb"),
    JsonbArray = (3807, "_jsonb"),
    AnyRange = (3831, "anyrange"),
    EventTrigger = (3838, "event_trigger"),
    Int4Range = (3904, "int4range"),
    Int4RangeArray = (3905, "_int4range"),
    NumRange = (3906, "numrange"),
    NumRangeArray = (3907, "_numrange"),
    TsRange = (3908, "tsrange"),
    TsRangeArray = (3909, "_tsrange"),
    TstzRange = (3910, "tstzrange"),
    TstzRangeArray = (3911, "_tstzrange"),
    DateRange = (3912, "daterange"),
    DateRangeArray = (3913, "_daterange"),
    Int8Range = (3926, "int8range"),
    Int8RangeArray = (3927, "_int8range"),
    Jsonpath = (4072, "jsonpath"),
    JsonpathArray = (4073, "_jsonpath"),
    Regnamespace = (4089, "regnamespace"),
    RegnamespaceArray = (4090, "_regnamespace"),
    Regrole = (4096, "regrole"),
    RegroleArray = (4097, "_regrole"),
    Regcollation = (4191, "regcollation"),
    RegcollationArray = (4192, "_regcollation"),
    Int4multiRange = (4451, "int4multirange"),
    NummultiRange = (4532, "nummultirange"),
    TsmultiRange = (4533, "tsmultirange"),
    TstzmultiRange = (4534, "tstzmultirange"),
    DatemultiRange = (4535, "datemultirange"),
    Int8multiRange = (4536, "int8multirange"),
    AnymultiRange = (4537, "anymultirange"),
    AnycompatiblemultiRange = (4538, "anycompatiblemultirange"),
    PgBrinBloomSummary = (4600, "pg_brin_bloom_summary"),
    PgBrinMinmaxMultiSummary = (4601, "pg_brin_minmax_multi_summary"),
    PgMcvList = (5017, "pg_mcv_list"),
    PgSnapshot = (5038, "pg_snapshot"),
    PgSnapshotArray = (5039, "_pg_snapshot"),
    Xid8 = (5069, "xid8"),
    Anycompatible = (5077, "anycompatible"),
    Anycompatiblearray = (5078, "anycompatiblearray"),
    Anycompatiblenonarray = (5079, "anycompatiblenonarray"),
    AnycompatibleRange = (5080, "anycompatiblerange"),
    Int4multiRangeArray = (6150, "_int4multirange"),
    NummultiRangeArray = (6151, "_nummultirange"),
    TsmultiRangeArray = (6152, "_tsmultirange"),
    TstzmultiRangeArray = (6153, "_tstzmultirange"),
    DatemultiRangeArray = (6155, "_datemultirange"),
    Int8multiRangeArray = (6157, "_int8multirange")
  }
}
