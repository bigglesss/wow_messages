## Client Version 1.12

```rust,ignore
enum Area : u32 {
    DUN_MOROGH = 1;    
    LONGSHORE = 2;    
    BADLANDS = 3;    
    BLASTED_LANDS = 4;    
    BLACKWATER_COVE = 7;    
    SWAMP_OF_SORROWS = 8;    
    NORTHSHIRE_VALLEY = 9;    
    DUSKWOOD = 10;    
    WETLANDS = 11;    
    ELWYNN_FOREST = 12;    
    THE_WORLD_TREE = 13;    
    DUROTAR = 14;    
    DUSTWALLOW_MARSH = 15;    
    AZSHARA = 16;    
    THE_BARRENS = 17;    
    CRYSTAL_LAKE = 18;    
    ZUL_GURUB0 = 19;    
    MOONBROOK = 20;    
    KUL_TIRAS = 21;    
    PROGRAMMER_ISLE = 22;    
    NORTHSHIRE_RIVER = 23;    
    NORTHSHIRE_ABBEY = 24;    
    BLACKROCK_MOUNTAIN0 = 25;    
    LIGHTHOUSE = 26;    
    WESTERN_PLAGUELANDS = 28;    
    NINE = 30;    
    THE_CEMETARY = 32;    
    STRANGLETHORN_VALE = 33;    
    ECHO_RIDGE_MINE = 34;    
    BOOTY_BAY = 35;    
    ALTERAC_MOUNTAINS = 36;    
    LAKE_NAZFERITI = 37;    
    LOCH_MODAN = 38;    
    WESTFALL0 = 40;    
    DEADWIND_PASS = 41;    
    DARKSHIRE = 42;    
    WILD_SHORE = 43;    
    REDRIDGE_MOUNTAINS = 44;    
    ARATHI_HIGHLANDS = 45;    
    BURNING_STEPPES = 46;    
    THE_HINTERLANDS = 47;    
    DEAD_MANS_HOLE = 49;    
    SEARING_GORGE = 51;    
    THIEVES_CAMP = 53;    
    JASPERLODE_MINE = 54;    
    VALLEY_OF_HEROES_UNUSED = 55;    
    HEROES_VIGIL = 56;    
    FARGODEEP_MINE = 57;    
    NORTHSHIRE_VINEYARDS = 59;    
    FORESTS_EDGE = 60;    
    THUNDER_FALLS = 61;    
    BRACKWELL_PUMPKIN_PATCH = 62;    
    THE_STONEFIELD_FARM = 63;    
    THE_MACLURE_VINEYARDS = 64;    
    ON_MAP_DUNGEON0 = 65;    
    ON_MAP_DUNGEON1 = 66;    
    ON_MAP_DUNGEON2 = 67;    
    LAKE_EVERSTILL = 68;    
    LAKESHIRE = 69;    
    STONEWATCH = 70;    
    STONEWATCH_FALLS = 71;    
    THE_DARK_PORTAL = 72;    
    THE_TAINTED_SCAR = 73;    
    POOL_OF_TEARS = 74;    
    STONARD = 75;    
    FALLOW_SANCTUARY = 76;    
    ANVILMAR = 77;    
    STORMWIND_MOUNTAINS = 80;    
    JEFF_NE_QUADRANT_CHANGED = 81;    
    JEFF_NW_QUADRANT = 82;    
    JEFF_SE_QUADRANT = 83;    
    JEFF_SW_QUADRANT = 84;    
    TIRISFAL_GLADES = 85;    
    STONE_CAIRN_LAKE = 86;    
    GOLDSHIRE = 87;    
    EASTVALE_LOGGING_CAMP = 88;    
    MIRROR_LAKE_ORCHARD = 89;    
    TOWER_OF_AZORA = 91;    
    MIRROR_LAKE = 92;    
    VUL_GOL_OGRE_MOUND = 93;    
    RAVEN_HILL = 94;    
    REDRIDGE_CANYONS = 95;    
    TOWER_OF_ILGALAR = 96;    
    ALTHERS_MILL = 97;    
    RETHBAN_CAVERNS = 98;    
    REBEL_CAMP = 99;    
    NESINGWARYS_EXPEDITION = 100;    
    KURZENS_COMPOUND = 101;    
    RUINS_OF_ZUL_KUNDA = 102;    
    RUINS_OF_ZUL_MAMWE = 103;    
    THE_VILE_REEF = 104;    
    MOSH_OGG_OGRE_MOUND = 105;    
    THE_STOCKPILE = 106;    
    SALDEANS_FARM = 107;    
    SENTINEL_HILL = 108;    
    FURLBROWS_PUMPKIN_FARM = 109;    
    JANGOLODE_MINE = 111;    
    GOLD_COAST_QUARRY = 113;    
    WESTFALL_LIGHTHOUSE = 115;    
    MISTY_VALLEY = 116;    
    GROMGOL_BASE_CAMP = 117;    
    WHELGARS_EXCAVATION_SITE = 118;    
    WESTBROOK_GARRISON = 120;    
    TRANQUIL_GARDENS_CEMETERY = 121;    
    ZUULDAIA_RUINS = 122;    
    BALLAL_RUINS = 123;    
    KALAI_RUINS = 125;    
    TKASHI_RUINS = 126;    
    BALIAMAH_RUINS = 127;    
    ZIATAJAI_RUINS = 128;    
    MIZJAH_RUINS = 129;    
    SILVERPINE_FOREST = 130;    
    KHARANOS = 131;    
    COLDRIDGE_VALLEY = 132;    
    GNOMEREGAN0 = 133;    
    GOL_BOLAR_QUARRY = 134;    
    FROSTMANE_HOLD = 135;    
    THE_GRIZZLED_DEN = 136;    
    BREWNALL_VILLAGE = 137;    
    MISTY_PINE_REFUGE = 138;    
    EASTERN_PLAGUELANDS = 139;    
    TELDRASSIL = 141;    
    IRONBANDS_EXCAVATION_SITE = 142;    
    MOGROSH_STRONGHOLD = 143;    
    THELSAMAR = 144;    
    ALGAZ_GATE = 145;    
    STONEWROUGHT_DAM = 146;    
    THE_FARSTRIDER_LODGE = 147;    
    DARKSHORE = 148;    
    SILVER_STREAM_MINE = 149;    
    MENETHIL_HARBOR = 150;    
    DESIGNER_ISLAND = 151;    
    THE_BULWARK0 = 152;    
    RUINS_OF_LORDAERON = 153;    
    DEATHKNELL = 154;    
    NIGHT_WEBS_HOLLOW = 155;    
    SOLLIDEN_FARMSTEAD = 156;    
    AGAMAND_MILLS = 157;    
    AGAMAND_FAMILY_CRYPT = 158;    
    BRILL = 159;    
    WHISPERING_GARDENS = 160;    
    TERRACE_OF_REPOSE = 161;    
    BRIGHTWATER_LAKE = 162;    
    GUNTHERS_RETREAT = 163;    
    GARRENS_HAUNT = 164;    
    BALNIR_FARMSTEAD = 165;    
    COLD_HEARTH_MANOR = 166;    
    CRUSADER_OUTPOST = 167;    
    THE_NORTH_COAST = 168;    
    WHISPERING_SHORE = 169;    
    LORDAMERE_LAKE0 = 170;    
    FENRIS_ISLE = 172;    
    FAOLS_REST = 173;    
    DOLANAAR = 186;    
    DARNASSUS_UNUSED = 187;    
    SHADOWGLEN = 188;    
    STEELGRILLS_DEPOT = 189;    
    HEARTHGLEN = 190;    
    NORTHRIDGE_LUMBER_CAMP = 192;    
    RUINS_OF_ANDORHAL = 193;    
    SCHOOL_OF_NECROMANCY = 195;    
    UTHERS_TOMB = 196;    
    SORROW_HILL = 197;    
    THE_WEEPING_CAVE = 198;    
    FELSTONE_FIELD = 199;    
    DALSONS_TEARS = 200;    
    GAHRRONS_WITHERING = 201;    
    THE_WRITHING_HAUNT = 202;    
    MARDENHOLDE_KEEP = 203;    
    PYREWOOD_VILLAGE = 204;    
    DUN_MODR = 205;    
    WESTFALL1 = 206;    
    THE_GREAT_SEA0 = 207;    
    UNUSED_IRONCLADCOVE = 208;    
    SHADOWFANG_KEEP0 = 209;    
    ON_MAP_DUNGEON3 = 210;    
    ICEFLOW_LAKE = 211;    
    HELMS_BED_LAKE = 212;    
    DEEP_ELEM_MINE = 213;    
    THE_GREAT_SEA1 = 214;    
    MULGORE = 215;    
    ALEXSTON_FARMSTEAD = 219;    
    RED_CLOUD_MESA = 220;    
    CAMP_NARACHE = 221;    
    BLOODHOOF_VILLAGE = 222;    
    STONEBULL_LAKE = 223;    
    RAVAGED_CARAVAN = 224;    
    RED_ROCKS = 225;    
    THE_SKITTERING_DARK = 226;    
    VALGANS_FIELD = 227;    
    THE_SEPULCHER = 228;    
    OLSENS_FARTHING = 229;    
    THE_GREYMANE_WALL = 230;    
    BERENS_PERIL = 231;    
    THE_DAWNING_ISLES = 232;    
    AMBERMILL = 233;    
    FENRIS_KEEP = 235;    
    SHADOWFANG_KEEP1 = 236;    
    THE_DECREPIT_FERRY = 237;    
    MALDENS_ORCHARD = 238;    
    THE_IVAR_PATCH = 239;    
    THE_DEAD_FIELD = 240;    
    THE_ROTTING_ORCHARD = 241;    
    BRIGHTWOOD_GROVE = 242;    
    FORLORN_ROWE = 243;    
    THE_WHIPPLE_ESTATE = 244;    
    THE_YORGEN_FARMSTEAD = 245;    
    THE_CAULDRON = 246;    
    GRIMESILT_DIG_SITE = 247;    
    DREADMAUL_ROCK = 249;    
    RUINS_OF_THAURISSAN = 250;    
    FLAME_CREST = 251;    
    BLACKROCK_STRONGHOLD = 252;    
    THE_PILLAR_OF_ASH = 253;    
    BLACKROCK_MOUNTAIN1 = 254;    
    ALTAR_OF_STORMS0 = 255;    
    ALDRASSIL = 256;    
    SHADOWTHREAD_CAVE = 257;    
    FEL_ROCK = 258;    
    LAKE_AL_AMETH = 259;    
    STARBREEZE_VILLAGE = 260;    
    GNARLPINE_HOLD = 261;    
    BANETHIL_BARROW_DEN = 262;    
    THE_CLEFT = 263;    
    THE_ORACLE_GLADE = 264;    
    WELLSPRING_RIVER = 265;    
    WELLSPRING_LAKE = 266;    
    HILLSBRAD_FOOTHILLS = 267;    
    AZSHARA_CRATER = 268;    
    DUN_ALGAZ0 = 269;    
    SOUTHSHORE0 = 271;    
    TARREN_MILL0 = 272;    
    DURNHOLDE_KEEP0 = 275;    
    UNUSED_STONEWROUGHT_PASS = 276;    
    THE_FOOTHILL_CAVERNS = 277;    
    LORDAMERE_INTERNMENT_CAMP = 278;    
    DALARAN = 279;    
    STRAHNBRAD = 280;    
    RUINS_OF_ALTERAC = 281;    
    CRUSHRIDGE_HOLD = 282;    
    SLAUGHTER_HOLLOW = 283;    
    THE_UPLANDS = 284;    
    SOUTHPOINT_TOWER0 = 285;    
    HILLSBRAD_FIELDS0 = 286;    
    HILLSBRAD = 287;    
    AZURELODE_MINE0 = 288;    
    NETHANDER_STEAD0 = 289;    
    DUN_GAROK0 = 290;    
    THORADINS_WALL0 = 293;    
    EASTERN_STRAND0 = 294;    
    WESTERN_STRAND0 = 295;    
    SOUTH_SEAS_UNUSED = 296;    
    JAGUERO_ISLE = 297;    
    BARADIN_BAY = 298;    
    MENETHIL_BAY = 299;    
    MISTY_REED_STRAND = 300;    
    THE_SAVAGE_COAST = 301;    
    THE_CRYSTAL_SHORE = 302;    
    SHELL_BEACH = 303;    
    NORTH_TIDES_RUN = 305;    
    SOUTH_TIDES_RUN = 306;    
    THE_OVERLOOK_CLIFFS = 307;    
    THE_FORBIDDING_SEA0 = 308;    
    IRONBEARDS_TOMB = 309;    
    CRYSTALVEIN_MINE = 310;    
    RUINS_OF_ABORAZ = 311;    
    JANEIROS_POINT = 312;    
    NORTHFOLD_MANOR = 313;    
    GO_SHEK_FARM = 314;    
    DABYRIES_FARMSTEAD = 315;    
    BOULDERFIST_HALL = 316;    
    WITHERBARK_VILLAGE = 317;    
    DRYWHISKER_GORGE = 318;    
    REFUGE_POINTE = 320;    
    HAMMERFALL = 321;    
    BLACKWATER_SHIPWRECKS = 322;    
    O_BREENS_CAMP = 323;    
    STROMGARDE_KEEP = 324;    
    THE_TOWER_OF_ARATHOR = 325;    
    THE_SANCTUM = 326;    
    FALDIRS_COVE = 327;    
    THE_DROWNED_REEF = 328;    
    THANDOL_SPAN0 = 330;    
    ASHENVALE = 331;    
    THE_GREAT_SEA2 = 332;    
    CIRCLE_OF_EAST_BINDING = 333;    
    CIRCLE_OF_WEST_BINDING = 334;    
    CIRCLE_OF_INNER_BINDING = 335;    
    CIRCLE_OF_OUTER_BINDING = 336;    
    APOCRYPHANS_REST = 337;    
    ANGOR_FORTRESS = 338;    
    LETHLOR_RAVINE = 339;    
    KARGATH = 340;    
    CAMP_KOSH = 341;    
    CAMP_BOFF = 342;    
    CAMP_WURG = 343;    
    CAMP_CAGG = 344;    
    AGMONDS_END = 345;    
    HAMMERTOES_DIGSITE = 346;    
    DUSTBELCH_GROTTO = 347;    
    AERIE_PEAK = 348;    
    WILDHAMMER_KEEP = 349;    
    QUEL_DANIL_LODGE = 350;    
    SKULK_ROCK = 351;    
    ZUNWATHA = 352;    
    SHADRA_ALOR = 353;    
    JINTHA_ALOR = 354;    
    THE_ALTAR_OF_ZUL = 355;    
    SERADANE = 356;    
    FERALAS = 357;    
    BRAMBLEBLADE_RAVINE = 358;    
    BAEL_MODAN = 359;    
    THE_VENTURE_CO_MINE = 360;    
    FELWOOD = 361;    
    RAZOR_HILL = 362;    
    VALLEY_OF_TRIALS = 363;    
    THE_DEN = 364;    
    BURNING_BLADE_COVEN = 365;    
    KOLKAR_CRAG = 366;    
    SENJIN_VILLAGE = 367;    
    ECHO_ISLES = 368;    
    THUNDER_RIDGE = 369;    
    DRYGULCH_RAVINE = 370;    
    DUSTWIND_CAVE = 371;    
    TIRAGARDE_KEEP = 372;    
    SCUTTLE_COAST = 373;    
    BLADEFIST_BAY = 374;    
    DEADEYE_SHORE = 375;    
    SOUTHFURY_RIVER0 = 377;    
    CAMP_TAURAJO = 378;    
    FAR_WATCH_POST = 379;    
    THE_CROSSROADS = 380;    
    BOULDER_LODE_MINE = 381;    
    THE_SLUDGE_FEN = 382;    
    THE_DRY_HILLS = 383;    
    DREADMIST_PEAK = 384;    
    NORTHWATCH_HOLD = 385;    
    THE_FORGOTTEN_POOLS = 386;    
    LUSHWATER_OASIS = 387;    
    THE_STAGNANT_OASIS = 388;    
    FIELD_OF_GIANTS = 390;    
    THE_MERCHANT_COAST = 391;    
    RATCHET = 392;    
    DARKSPEAR_STRAND = 393;    
    DARROWMERE_LAKE_UNUSED = 394;    
    CAER_DARROW_UNUSED = 395;    
    WINTERHOOF_WATER_WELL = 396;    
    THUNDERHORN_WATER_WELL = 397;    
    WILDMANE_WATER_WELL = 398;    
    SKYLINE_RIDGE = 399;    
    THOUSAND_NEEDLES = 400;    
    THE_TIDUS_STAIR = 401;    
    SHADY_REST_INN = 403;    
    BAELDUN_DIGSITE = 404;    
    DESOLACE = 405;    
    STONETALON_MOUNTAINS = 406;    
    ORGRIMMAR_UNUSED = 407;    
    GILLIJIMS_ISLE = 408;    
    ISLAND_OF_DOCTOR_LAPIDIS = 409;    
    RAZORWIND_CANYON = 410;    
    BATHRANS_HAUNT = 411;    
    THE_RUINS_OF_ORDIL_ARAN = 412;    
    MAESTRAS_POST = 413;    
    THE_ZORAM_STRAND = 414;    
    ASTRANAAR = 415;    
    THE_SHRINE_OF_AESSINA = 416;    
    FIRE_SCAR_SHRINE = 417;    
    THE_RUINS_OF_STARDUST = 418;    
    THE_HOWLING_VALE = 419;    
    SILVERWIND_REFUGE = 420;    
    MYSTRAL_LAKE = 421;    
    FALLEN_SKY_LAKE = 422;    
    IRIS_LAKE = 424;    
    MOONWELL = 425;    
    RAYNEWOOD_RETREAT = 426;    
    THE_SHADY_NOOK = 427;    
    NIGHT_RUN = 428;    
    XAVIAN = 429;    
    SATYRNAAR = 430;    
    SPLINTERTREE_POST = 431;    
    THE_DOR_DANIL_BARROW_DEN = 432;    
    FALFARREN_RIVER = 433;    
    FELFIRE_HILL = 434;    
    DEMON_FALL_CANYON = 435;    
    DEMON_FALL_RIDGE = 436;    
    WARSONG_LUMBER_CAMP = 437;    
    BOUGH_SHADOW = 438;    
    THE_SHIMMERING_FLATS = 439;    
    TANARIS = 440;    
    LAKE_FALATHIM = 441;    
    AUBERDINE = 442;    
    RUINS_OF_MATHYSTRA = 443;    
    TOWER_OF_ALTHALAXX = 444;    
    CLIFFSPRING_FALLS = 445;    
    BASHAL_ARAN = 446;    
    AMETH_ARAN = 447;    
    GROVE_OF_THE_ANCIENTS = 448;    
    THE_MASTERS_GLAIVE = 449;    
    REMTRAVELS_EXCAVATION = 450;    
    MISTS_EDGE = 452;    
    THE_LONG_WASH = 453;    
    WILDBEND_RIVER = 454;    
    BLACKWOOD_DEN = 455;    
    CLIFFSPRING_RIVER = 456;    
    THE_VEILED_SEA0 = 457;    
    GOLD_ROAD = 458;    
    SCARLET_WATCH_POST = 459;    
    SUN_ROCK_RETREAT = 460;    
    WINDSHEAR_CRAG = 461;    
    CRAGPOOL_LAKE = 463;    
    MIRKFALLON_LAKE = 464;    
    THE_CHARRED_VALE = 465;    
    VALLEY_OF_THE_BLOODFURIES = 466;    
    STONETALON_PEAK = 467;    
    THE_TALON_DEN = 468;    
    GREATWOOD_VALE = 469;    
    THUNDER_BLUFF_UNUSED = 470;    
    BRAVE_WIND_MESA = 471;    
    FIRE_STONE_MESA = 472;    
    MANTLE_ROCK = 473;    
    HUNTER_RISE_UNUSED = 474;    
    SPIRIT_RISE_UNUSED = 475;    
    ELDER_RISE_UNUSED = 476;    
    RUINS_OF_JUBUWAL = 477;    
    POOLS_OF_ARLITHRIEN = 478;    
    THE_RUSTMAUL_DIG_SITE = 479;    
    CAMP_ETHOK = 480;    
    SPLITHOOF_CRAG = 481;    
    HIGHPERCH = 482;    
    THE_SCREECHING_CANYON = 483;    
    FREEWIND_POST = 484;    
    THE_GREAT_LIFT0 = 485;    
    GALAK_HOLD = 486;    
    ROGUEFEATHER_DEN = 487;    
    THE_WEATHERED_NOOK = 488;    
    THALANAAR = 489;    
    UN_GORO_CRATER = 490;    
    RAZORFEN_KRAUL0 = 491;    
    RAVEN_HILL_CEMETERY = 492;    
    MOONGLADE = 493;    
    DELETE_ME0 = 495;    
    BRACKENWALL_VILLAGE = 496;    
    SWAMPLIGHT_MANOR = 497;    
    BLOODFEN_BURROW = 498;    
    DARKMIST_CAVERN = 499;    
    MOGGLE_POINT = 500;    
    BEEZILS_WRECK = 501;    
    WITCH_HILL = 502;    
    SENTRY_POINT = 503;    
    NORTH_POINT_TOWER = 504;    
    WEST_POINT_TOWER = 505;    
    LOST_POINT = 506;    
    BLUEFEN = 507;    
    STONEMAUL_RUINS = 508;    
    THE_DEN_OF_FLAME = 509;    
    THE_DRAGONMURK = 510;    
    WYRMBOG = 511;    
    ONYXIAS_LAIR_UNUSED = 512;    
    THERAMORE_ISLE = 513;    
    FOOTHOLD_CITADEL = 514;    
    IRONCLAD_PRISON = 515;    
    DUSTWALLOW_BAY = 516;    
    TIDEFURY_COVE = 517;    
    DREADMURK_SHORE = 518;    
    ADDLES_STEAD = 536;    
    FIRE_PLUME_RIDGE = 537;    
    LAKKARI_TAR_PITS = 538;    
    TERROR_RUN = 539;    
    THE_SLITHERING_SCAR = 540;    
    MARSHALS_REFUGE = 541;    
    FUNGAL_ROCK = 542;    
    GOLAKKA_HOT_SPRINGS = 543;    
    THE_LOCH = 556;    
    BEGGARS_HAUNT = 576;    
    KODO_GRAVEYARD = 596;    
    GHOST_WALKER_POST = 597;    
    SARTHERIS_STRAND = 598;    
    THUNDER_AXE_FORTRESS = 599;    
    BOLGANS_HOLE = 600;    
    MANNOROC_COVEN = 602;    
    SARGERON = 603;    
    MAGRAM_VILLAGE = 604;    
    GELKIS_VILLAGE = 606;    
    VALLEY_OF_SPEARS = 607;    
    NIJELS_POINT = 608;    
    KOLKAR_VILLAGE = 609;    
    HYJAL = 616;    
    WINTERSPRING = 618;    
    BLACKWOLF_RIVER = 636;    
    KODO_ROCK = 637;    
    HIDDEN_PATH = 638;    
    SPIRIT_ROCK = 639;    
    SHRINE_OF_THE_DORMANT_FLAME = 640;    
    LAKE_ELUNEARA = 656;    
    THE_HARBORAGE = 657;    
    OUTLAND = 676;    
    CRAFTSMENS_TERRACE_UNUSED = 696;    
    TRADESMENS_TERRACE_UNUSED = 697;    
    THE_TEMPLE_GARDENS_UNUSED = 698;    
    TEMPLE_OF_ELUNE_UNUSED = 699;    
    CENARION_ENCLAVE_UNUSED = 700;    
    WARRIORS_TERRACE_UNUSED = 701;    
    RUTTHERAN_VILLAGE = 702;    
    IRONBANDS_COMPOUND = 716;    
    THE_STOCKADE = 717;    
    WAILING_CAVERNS = 718;    
    BLACKFATHOM_DEEPS0 = 719;    
    FRAY_ISLAND = 720;    
    GNOMEREGAN1 = 721;    
    RAZORFEN_DOWNS0 = 722;    
    BANETHIL_HOLLOW = 736;    
    SCARLET_MONASTERY = 796;    
    JERODS_LANDING = 797;    
    RIDGEPOINT_TOWER = 798;    
    THE_DARKENED_BANK = 799;    
    COLDRIDGE_PASS = 800;    
    CHILL_BREEZE_VALLEY = 801;    
    SHIMMER_RIDGE = 802;    
    AMBERSTILL_RANCH = 803;    
    THE_TUNDRID_HILLS = 804;    
    SOUTH_GATE_PASS0 = 805;    
    SOUTH_GATE_OUTPOST = 806;    
    NORTH_GATE_PASS0 = 807;    
    NORTH_GATE_OUTPOST = 808;    
    GATES_OF_IRONFORGE = 809;    
    STILLWATER_POND = 810;    
    NIGHTMARE_VALE = 811;    
    VENOMWEB_VALE = 812;    
    THE_BULWARK1 = 813;    
    SOUTHFURY_RIVER1 = 814;    
    SOUTHFURY_RIVER2 = 815;    
    RAZORMANE_GROUNDS = 816;    
    SKULL_ROCK = 817;    
    PALEMANE_ROCK = 818;    
    WINDFURY_RIDGE = 819;    
    THE_GOLDEN_PLAINS = 820;    
    THE_ROLLING_PLAINS = 821;    
    DUN_ALGAZ1 = 836;    
    DUN_ALGAZ2 = 837;    
    NORTH_GATE_PASS1 = 838;    
    SOUTH_GATE_PASS1 = 839;    
    TWILIGHT_GROVE = 856;    
    GM_ISLAND = 876;    
    DELETE_ME1 = 877;    
    SOUTHFURY_RIVER3 = 878;    
    SOUTHFURY_RIVER4 = 879;    
    THANDOL_SPAN1 = 880;    
    THANDOL_SPAN2 = 881;    
    PURGATION_ISLE = 896;    
    THE_JANSEN_STEAD = 916;    
    THE_DEAD_ACRE = 917;    
    THE_MOLSEN_FARM = 918;    
    STENDELS_POND = 919;    
    THE_DAGGER_HILLS = 920;    
    DEMONTS_PLACE = 921;    
    THE_DUST_PLAINS = 922;    
    STONESPLINTER_VALLEY = 923;    
    VALLEY_OF_KINGS = 924;    
    ALGAZ_STATION = 925;    
    BUCKLEBREE_FARM = 926;    
    THE_SHINING_STRAND = 927;    
    NORTH_TIDES_HOLLOW = 928;    
    GRIZZLEPAW_RIDGE = 936;    
    THE_VERDANT_FIELDS = 956;    
    GADGETZAN = 976;    
    STEAMWHEEDLE_PORT = 977;    
    ZUL_FARRAK0 = 978;    
    SANDSORROW_WATCH = 979;    
    THISTLESHRUB_VALLEY = 980;    
    THE_GAPING_CHASM = 981;    
    THE_NOXIOUS_LAIR = 982;    
    DUNEMAUL_COMPOUND = 983;    
    EASTMOON_RUINS = 984;    
    WATERSPRING_FIELD = 985;    
    ZALASHJIS_DEN = 986;    
    LANDS_END_BEACH = 987;    
    WAVESTRIDER_BEACH = 988;    
    ULDUM = 989;    
    VALLEY_OF_THE_WATCHERS = 990;    
    GUNSTANS_POST = 991;    
    SOUTHMOON_RUINS = 992;    
    RENDERS_CAMP = 996;    
    RENDERS_VALLEY = 997;    
    RENDERS_ROCK = 998;    
    STONEWATCH_TOWER = 999;    
    GALARDELL_VALLEY = 1000;    
    LAKERIDGE_HIGHWAY = 1001;    
    THREE_CORNERS = 1002;    
    DIREFORGE_HILL = 1016;    
    RAPTOR_RIDGE = 1017;    
    BLACK_CHANNEL_MARSH = 1018;    
    THE_GREEN_BELT0 = 1019;    
    MOSSHIDE_FEN = 1020;    
    THELGEN_ROCK = 1021;    
    BLUEGILL_MARSH = 1022;    
    SALTSPRAY_GLEN = 1023;    
    SUNDOWN_MARSH = 1024;    
    THE_GREEN_BELT1 = 1025;    
    ANGERFANG_ENCAMPMENT = 1036;    
    GRIM_BATOL = 1037;    
    DRAGONMAW_GATES = 1038;    
    THE_LOST_FLEET = 1039;    
    DARROW_HILL0 = 1056;    
    THORADINS_WALL1 = 1057;    
    WEBWINDER_PATH = 1076;    
    THE_HUSHED_BANK = 1097;    
    MANOR_MISTMANTLE = 1098;    
    CAMP_MOJACHE = 1099;    
    GRIMTOTEM_COMPOUND = 1100;    
    THE_WRITHING_DEEP = 1101;    
    WILDWIND_LAKE = 1102;    
    GORDUNNI_OUTPOST = 1103;    
    MOK_GORDUN = 1104;    
    FERAL_SCAR_VALE = 1105;    
    FRAYFEATHER_HIGHLANDS = 1106;    
    IDLEWIND_LAKE = 1107;    
    THE_FORGOTTEN_COAST = 1108;    
    EAST_PILLAR = 1109;    
    WEST_PILLAR = 1110;    
    DREAM_BOUGH = 1111;    
    JADEMIR_LAKE = 1112;    
    ONEIROS = 1113;    
    RUINS_OF_RAVENWIND = 1114;    
    RAGE_SCAR_HOLD = 1115;    
    FEATHERMOON_STRONGHOLD = 1116;    
    RUINS_OF_SOLARSAL = 1117;    
    LOWER_WILDS_UNUSED = 1118;    
    THE_TWIN_COLOSSALS = 1119;    
    SARDOR_ISLE = 1120;    
    ISLE_OF_DREAD = 1121;    
    HIGH_WILDERNESS = 1136;    
    LOWER_WILDS = 1137;    
    SOUTHERN_BARRENS = 1156;    
    SOUTHERN_GOLD_ROAD = 1157;    
    ZUL_FARRAK1 = 1176;    
    UNUSED_ALCAZ_ISLAND = 1196;    
    TIMBERMAW_HOLD0 = 1216;    
    VANNDIR_ENCAMPMENT = 1217;    
    TEST_AZSHARA = 1218;    
    LEGASH_ENCAMPMENT = 1219;    
    THALASSIAN_BASE_CAMP = 1220;    
    RUINS_OF_ELDARATH = 1221;    
    HETAERAS_CLUTCH = 1222;    
    TEMPLE_OF_ZIN_MALOR = 1223;    
    BEARS_HEAD = 1224;    
    URSOLAN = 1225;    
    TEMPLE_OF_ARKKORAN = 1226;    
    BAY_OF_STORMS = 1227;    
    THE_SHATTERED_STRAND = 1228;    
    TOWER_OF_ELDARA = 1229;    
    JAGGED_REEF = 1230;    
    SOUTHRIDGE_BEACH = 1231;    
    RAVENCREST_MONUMENT = 1232;    
    FORLORN_RIDGE = 1233;    
    LAKE_MENNAR = 1234;    
    SHADOWSONG_SHRINE = 1235;    
    HALDARR_ENCAMPMENT = 1236;    
    VALORMOK = 1237;    
    THE_RUINED_REACHES = 1256;    
    THE_TALONDEEP_PATH0 = 1276;    
    THE_TALONDEEP_PATH1 = 1277;    
    ROCKTUSK_FARM = 1296;    
    JAGGEDSWINE_FARM = 1297;    
    RAZORFEN_DOWNS1 = 1316;    
    LOST_RIGGER_COVE = 1336;    
    ULDAMAN0 = 1337;    
    LORDAMERE_LAKE1 = 1338;    
    LORDAMERE_LAKE2 = 1339;    
    GALLOWS_CORNER = 1357;    
    SILITHUS = 1377;    
    EMERALD_FOREST = 1397;    
    SUNKEN_TEMPLE = 1417;    
    DREADMAUL_HOLD = 1437;    
    NETHERGARDE_KEEP = 1438;    
    DREADMAUL_POST = 1439;    
    SERPENTS_COIL = 1440;    
    ALTAR_OF_STORMS1 = 1441;    
    FIREWATCH_RIDGE = 1442;    
    THE_SLAG_PIT = 1443;    
    THE_SEA_OF_CINDERS = 1444;    
    BLACKROCK_MOUNTAIN2 = 1445;    
    THORIUM_POINT = 1446;    
    GARRISON_ARMORY = 1457;    
    THE_TEMPLE_OF_ATAL_HAKKAR = 1477;    
    UNDERCITY = 1497;    
    ULDAMAN1 = 1517;    
    NOT_USED_DEADMINES = 1518;    
    STORMWIND_CITY = 1519;    
    IRONFORGE = 1537;    
    SPLITHOOF_HOLD = 1557;    
    THE_CAPE_OF_STRANGLETHORN = 1577;    
    SOUTHERN_SAVAGE_COAST = 1578;    
    UNUSED_THE_DEADMINES_002 = 1579;    
    UNUSED_IRONCLAD_COVE_003 = 1580;    
    THE_DEADMINES = 1581;    
    IRONCLAD_COVE = 1582;    
    BLACKROCK_SPIRE = 1583;    
    BLACKROCK_DEPTHS = 1584;    
    RAPTOR_GROUNDS_UNUSED = 1597;    
    GROLDOM_FARM_UNUSED = 1598;    
    MORSHAN_BASE_CAMP = 1599;    
    HONORS_STAND_UNUSED = 1600;    
    BLACKTHORN_RIDGE_UNUSED = 1601;    
    BRAMBLESCAR_UNUSED = 1602;    
    AGAMAGOR_UNUSED = 1603;    
    VALLEY_OF_HEROES = 1617;    
    ORGRIMMAR = 1637;    
    THUNDER_BLUFF = 1638;    
    ELDER_RISE = 1639;    
    SPIRIT_RISE = 1640;    
    HUNTER_RISE = 1641;    
    DARNASSUS = 1657;    
    CENARION_ENCLAVE = 1658;    
    CRAFTSMENS_TERRACE = 1659;    
    WARRIORS_TERRACE = 1660;    
    THE_TEMPLE_GARDENS = 1661;    
    TRADESMENS_TERRACE = 1662;    
    GAVINS_NAZE = 1677;    
    SOFERAS_NAZE = 1678;    
    CORRAHNS_DAGGER = 1679;    
    THE_HEADLAND = 1680;    
    MISTY_SHORE = 1681;    
    DANDREDS_FOLD = 1682;    
    GROWLESS_CAVE = 1683;    
    CHILLWIND_POINT = 1684;    
    RAPTOR_GROUNDS = 1697;    
    BRAMBLESCAR = 1698;    
    THORN_HILL = 1699;    
    AGAMAGOR = 1700;    
    BLACKTHORN_RIDGE = 1701;    
    HONORS_STAND = 1702;    
    THE_MORSHAN_RAMPART = 1703;    
    GROLDOM_FARM = 1704;    
    RAZORFEN_KRAUL1 = 1717;    
    THE_GREAT_LIFT1 = 1718;    
    MISTVALE_VALLEY = 1737;    
    NEKMANI_WELLSPRING = 1738;    
    BLOODSAIL_COMPOUND = 1739;    
    VENTURE_CO_BASE_CAMP = 1740;    
    GURUBASHI_ARENA = 1741;    
    SPIRIT_DEN = 1742;    
    THE_CRIMSON_VEIL = 1757;    
    THE_RIPTIDE = 1758;    
    THE_DAMSELS_LUCK = 1759;    
    VENTURE_CO_OPERATIONS_CENTER = 1760;    
    DEADWOOD_VILLAGE = 1761;    
    FELPAW_VILLAGE = 1762;    
    JAEDENAR = 1763;    
    BLOODVENOM_RIVER = 1764;    
    BLOODVENOM_FALLS = 1765;    
    SHATTER_SCAR_VALE = 1766;    
    IRONTREE_WOODS = 1767;    
    IRONTREE_CAVERN = 1768;    
    TIMBERMAW_HOLD1 = 1769;    
    SHADOW_HOLD = 1770;    
    SHRINE_OF_THE_DECEIVER = 1771;    
    ITHARIUSS_CAVE = 1777;    
    SORROWMURK = 1778;    
    DRAENILDUR_VILLAGE = 1779;    
    SPLINTERSPEAR_JUNCTION = 1780;    
    STAGALBOG = 1797;    
    THE_SHIFTING_MIRE = 1798;    
    STAGALBOG_CAVE = 1817;    
    WITHERBARK_CAVERNS = 1837;    
    THORADINS_WALL2 = 1857;    
    BOULDERGOR = 1858;    
    VALLEY_OF_FANGS = 1877;    
    THE_DUSTBOWL = 1878;    
    MIRAGE_FLATS = 1879;    
    FEATHERBEARDS_HOVEL = 1880;    
    SHINDIGGERS_CAMP = 1881;    
    PLAGUEMIST_RAVINE = 1882;    
    VALORWIND_LAKE = 1883;    
    AGOLWATHA = 1884;    
    HIRIWATHA = 1885;    
    THE_CREEPING_RUIN = 1886;    
    BOGENS_LEDGE = 1887;    
    THE_MAKERS_TERRACE = 1897;    
    DUSTWIND_GULCH = 1898;    
    SHAOLWATHA = 1917;    
    NOONSHADE_RUINS = 1937;    
    BROKEN_PILLAR = 1938;    
    ABYSSAL_SANDS = 1939;    
    SOUTHBREAK_SHORE = 1940;    
    CAVERNS_OF_TIME0 = 1941;    
    THE_MARSHLANDS = 1942;    
    IRONSTONE_PLATEAU = 1943;    
    BLACKCHAR_CAVE = 1957;    
    TANNER_CAMP = 1958;    
    DUSTFIRE_VALLEY = 1959;    
    ZUL_GURUB1 = 1977;    
    MISTY_REED_POST = 1978;    
    BLOODVENOM_POST = 1997;    
    TALONBRANCH_GLADE = 1998;    
    STRATHOLME0 = 2017;    
    UNUSED_SHADOWFANG_KEEP_003 = 2037;    
    SCHOLOMANCE = 2057;    
    TWILIGHT_VALE = 2077;    
    TWILIGHT_SHORE = 2078;    
    ALCAZ_ISLAND = 2079;    
    DARKCLOUD_PINNACLE = 2097;    
    DAWNING_WOOD_CATACOMBS = 2098;    
    STONEWATCH_KEEP = 2099;    
    MARAUDON = 2100;    
    STOUTLAGER_INN = 2101;    
    THUNDERBREW_DISTILLERY = 2102;    
    MENETHIL_KEEP = 2103;    
    DEEPWATER_TAVERN = 2104;    
    SHADOW_GRAVE = 2117;    
    BRILL_TOWN_HALL = 2118;    
    GALLOWS_END_TAVERN = 2119;    
    THE_POOLS_OF_VISION_UNUSED = 2137;    
    DREADMIST_DEN = 2138;    
    BAELDUN_KEEP = 2157;    
    EMBERSTRIFES_DEN = 2158;    
    ONYXIAS_LAIR = 2159;    
    WINDSHEAR_MINE = 2160;    
    ROLANDS_DOOM = 2161;    
    BATTLE_RING = 2177;    
    THE_POOLS_OF_VISION = 2197;    
    SHADOWBREAK_RAVINE = 2198;    
    BROKEN_SPEAR_VILLAGE = 2217;    
    WHITEREACH_POST = 2237;    
    GORNIA = 2238;    
    ZANES_EYE_CRATER = 2239;    
    MIRAGE_RACEWAY = 2240;    
    FROSTSABER_ROCK = 2241;    
    THE_HIDDEN_GROVE = 2242;    
    TIMBERMAW_POST = 2243;    
    WINTERFALL_VILLAGE = 2244;    
    MAZTHORIL = 2245;    
    FROSTFIRE_HOT_SPRINGS = 2246;    
    ICE_THISTLE_HILLS = 2247;    
    DUN_MANDARR = 2248;    
    FROSTWHISPER_GORGE = 2249;    
    OWL_WING_THICKET = 2250;    
    LAKE_KEL_THERIL = 2251;    
    THE_RUINS_OF_KEL_THERIL = 2252;    
    STARFALL_VILLAGE = 2253;    
    BAN_THALLOW_BARROW_DEN = 2254;    
    EVERLOOK = 2255;    
    DARKWHISPER_GORGE = 2256;    
    DEEPRUN_TRAM = 2257;    
    THE_FUNGAL_VALE = 2258;    
    UNUSED_THE_MARRIS_STEAD = 2259;    
    THE_MARRIS_STEAD = 2260;    
    THE_UNDERCROFT = 2261;    
    DARROWSHIRE = 2262;    
    CROWN_GUARD_TOWER = 2263;    
    CORINS_CROSSING = 2264;    
    SCARLET_BASE_CAMP = 2265;    
    TYRS_HAND = 2266;    
    THE_SCARLET_BASILICA = 2267;    
    LIGHTS_HOPE_CHAPEL = 2268;    
    BROWMAN_MILL = 2269;    
    THE_NOXIOUS_GLADE = 2270;    
    EASTWALL_TOWER = 2271;    
    NORTHDALE = 2272;    
    ZUL_MASHAR = 2273;    
    MAZRA_ALOR = 2274;    
    NORTHPASS_TOWER = 2275;    
    QUEL_LITHIEN_LODGE = 2276;    
    PLAGUEWOOD = 2277;    
    SCOURGEHOLD = 2278;    
    STRATHOLME1 = 2279;    
    UNUSED_STRATHOLME = 2280;    
    DARROWMERE_LAKE0 = 2297;    
    CAER_DARROW = 2298;    
    DARROWMERE_LAKE1 = 2299;    
    CAVERNS_OF_TIME1 = 2300;    
    THISTLEFUR_VILLAGE = 2301;    
    THE_QUAGMIRE = 2302;    
    WINDBREAK_CANYON = 2303;    
    SOUTH_SEAS0 = 2317;    
    THE_GREAT_SEA3 = 2318;    
    THE_GREAT_SEA4 = 2319;    
    THE_GREAT_SEA5 = 2320;    
    THE_GREAT_SEA6 = 2321;    
    THE_VEILED_SEA1 = 2322;    
    THE_VEILED_SEA2 = 2323;    
    THE_VEILED_SEA3 = 2324;    
    THE_VEILED_SEA4 = 2325;    
    THE_VEILED_SEA5 = 2326;    
    RAZOR_HILL_BARRACKS = 2337;    
    SOUTH_SEAS1 = 2338;    
    THE_GREAT_SEA7 = 2339;    
    BLOODTOOTH_CAMP = 2357;    
    FOREST_SONG = 2358;    
    GREENPAW_VILLAGE = 2359;    
    SILVERWING_OUTPOST = 2360;    
    NIGHTHAVEN = 2361;    
    SHRINE_OF_REMULOS = 2362;    
    STORMRAGE_BARROW_DENS = 2363;    
    THE_GREAT_SEA8 = 2364;    
    THE_GREAT_SEA9 = 2365;    
    THE_BLACK_MORASS = 2366;    
    OLD_HILLSBRAD_FOOTHILLS = 2367;    
    TARREN_MILL1 = 2368;    
    SOUTHSHORE1 = 2369;    
    DURNHOLDE_KEEP1 = 2370;    
    DUN_GAROK1 = 2371;    
    HILLSBRAD_FIELDS1 = 2372;    
    EASTERN_STRAND1 = 2373;    
    NETHANDER_STEAD1 = 2374;    
    DARROW_HILL1 = 2375;    
    SOUTHPOINT_TOWER1 = 2376;    
    THORADINS_WALL3 = 2377;    
    WESTERN_STRAND1 = 2378;    
    AZURELODE_MINE1 = 2379;    
    THE_GREAT_SEA10 = 2397;    
    THE_GREAT_SEA11 = 2398;    
    THE_GREAT_SEA12 = 2399;    
    THE_FORBIDDING_SEA1 = 2400;    
    THE_FORBIDDING_SEA2 = 2401;    
    THE_FORBIDDING_SEA3 = 2402;    
    THE_FORBIDDING_SEA4 = 2403;    
    TETHRIS_ARAN = 2404;    
    ETHEL_RETHOR = 2405;    
    RANAZJAR_ISLE = 2406;    
    KORMEKS_HUT = 2407;    
    SHADOWPREY_VILLAGE = 2408;    
    BLACKROCK_PASS = 2417;    
    MORGANS_VIGIL = 2418;    
    SLITHER_ROCK = 2419;    
    TERROR_WING_PATH = 2420;    
    DRACODAR = 2421;    
    RAGEFIRE_CHASM = 2437;    
    NIGHTSONG_WOODS = 2457;    
    THE_VEILED_SEA6 = 2477;    
    MORLOS_ARAN = 2478;    
    EMERALD_SANCTUARY = 2479;    
    JADEFIRE_GLEN = 2480;    
    RUINS_OF_CONSTELLAS = 2481;    
    BITTER_REACHES = 2497;    
    RISE_OF_THE_DEFILER = 2517;    
    LARISS_PAVILION = 2518;    
    WOODPAW_HILLS = 2519;    
    WOODPAW_DEN = 2520;    
    VERDANTIS_RIVER = 2521;    
    RUINS_OF_ISILDIEN = 2522;    
    GRIMTOTEM_POST = 2537;    
    CAMP_APARAJE = 2538;    
    MALAKAJIN = 2539;    
    BOULDERSLIDE_RAVINE = 2540;    
    SISHIR_CANYON = 2541;    
    DIRE_MAUL0 = 2557;    
    DEADWIND_RAVINE = 2558;    
    DIAMONDHEAD_RIVER = 2559;    
    ARIDENS_CAMP = 2560;    
    THE_VICE = 2561;    
    KARAZHAN = 2562;    
    MORGANS_PLOT = 2563;    
    DIRE_MAUL1 = 2577;    
    ALTERAC_VALLEY0 = 2597;    
    SCRABBLESCREWS_CAMP = 2617;    
    JADEFIRE_RUN = 2618;    
    THONDRORIL_RIVER0 = 2619;    
    THONDRORIL_RIVER1 = 2620;    
    LAKE_MERELDAR = 2621;    
    PESTILENT_SCAR = 2622;    
    THE_INFECTIS_SCAR = 2623;    
    BLACKWOOD_LAKE = 2624;    
    EASTWALL_GATE = 2625;    
    TERRORWEB_TUNNEL = 2626;    
    TERRORDALE = 2627;    
    KARGATHIA_KEEP = 2637;    
    VALLEY_OF_BONES = 2657;    
    BLACKWING_LAIR = 2677;    
    DEADMANS_CROSSING = 2697;    
    MOLTEN_CORE = 2717;    
    THE_SCARAB_WALL = 2737;    
    SOUTHWIND_VILLAGE = 2738;    
    TWILIGHT_BASE_CAMP = 2739;    
    THE_CRYSTAL_VALE = 2740;    
    THE_SCARAB_DAIS = 2741;    
    HIVE_ASHI = 2742;    
    HIVE_ZORA = 2743;    
    HIVE_REGAL = 2744;    
    SHRINE_OF_THE_FALLEN_WARRIOR = 2757;    
    UNUSED_ALTERAC_VALLEY = 2777;    
    BLACKFATHOM_DEEPS1 = 2797;    
    ON_MAP_DUNGEON4 = 2817;    
    THE_MASTERS_CELLAR = 2837;    
    STONEWROUGHT_PASS = 2838;    
    ALTERAC_VALLEY1 = 2839;    
    THE_RUMBLE_CAGE = 2857;    
    CHUNK_TEST = 2877;    
    ZORAMGAR_OUTPOST = 2897;    
    HALL_OF_LEGENDS = 2917;    
    CHAMPIONS_HALL = 2918;    
    GROSHGOK_COMPOUND = 2937;    
    SLEEPING_GORGE = 2938;    
    IRONDEEP_MINE = 2957;    
    STONEHEARTH_OUTPOST = 2958;    
    DUN_BALDAR = 2959;    
    ICEWING_PASS = 2960;    
    FROSTWOLF_VILLAGE = 2961;    
    TOWER_POINT = 2962;    
    COLDTOOTH_MINE = 2963;    
    WINTERAX_HOLD = 2964;    
    ICEBLOOD_GARRISON = 2977;    
    FROSTWOLF_KEEP = 2978;    
    TORKREN_FARM = 2979;    
    FROST_DAGGER_PASS = 3017;    
    IRONSTONE_CAMP = 3037;    
    WEAZELS_CRATER = 3038;    
    TAHONDA_RUINS = 3039;    
    FIELD_OF_STRIFE = 3057;    
    ICEWING_CAVERN = 3058;    
    VALORS_REST = 3077;    
    THE_SWARMING_PILLAR = 3097;    
    TWILIGHT_POST = 3098;    
    TWILIGHT_OUTPOST = 3099;    
    RAVAGED_TWILIGHT_CAMP = 3100;    
    SHALZARUS_LAIR = 3117;    
    TALRENDIS_POINT = 3137;    
    RETHRESS_SANCTUM = 3138;    
    MOON_HORROR_DEN = 3139;    
    SCALEBEARDS_CAVE = 3140;    
    BOULDERSLIDE_CAVERN = 3157;    
    WARSONG_LABOR_CAMP = 3177;    
    CHILLWIND_CAMP = 3197;    
    THE_MAUL = 3217;    
    THE_MAUL_UNUSED = 3237;    
    BONES_OF_GRAKKAROND = 3257;    
    WARSONG_GULCH = 3277;    
    FROSTWOLF_GRAVEYARD = 3297;    
    FROSTWOLF_PASS = 3298;    
    DUN_BALDAR_PASS = 3299;    
    ICEBLOOD_GRAVEYARD = 3300;    
    SNOWFALL_GRAVEYARD = 3301;    
    STONEHEARTH_GRAVEYARD = 3302;    
    STORMPIKE_GRAVEYARD = 3303;    
    ICEWING_BUNKER = 3304;    
    STONEHEARTH_BUNKER = 3305;    
    WILDPAW_RIDGE = 3306;    
    REVANTUSK_VILLAGE = 3317;    
    ROCK_OF_DUROTAN = 3318;    
    SILVERWING_GROVE = 3319;    
    WARSONG_LUMBER_MILL = 3320;    
    SILVERWING_HOLD = 3321;    
    WILDPAW_CAVERN = 3337;    
    THE_VEILED_CLEFT = 3338;    
    YOJAMBA_ISLE = 3357;    
    ARATHI_BASIN = 3358;    
    THE_COIL = 3377;    
    ALTAR_OF_HIREEK = 3378;    
    SHADRAZAAR = 3379;    
    HAKKARI_GROUNDS = 3380;    
    NAZE_OF_SHIRVALLAH = 3381;    
    TEMPLE_OF_BETHEKK = 3382;    
    THE_BLOODFIRE_PIT = 3383;    
    ALTAR_OF_THE_BLOOD_GOD = 3384;    
    ZANZAS_RISE = 3397;    
    EDGE_OF_MADNESS = 3398;    
    TROLLBANE_HALL = 3417;    
    DEFILERS_DEN = 3418;    
    PAGLES_POINTE = 3419;    
    FARM = 3420;    
    BLACKSMITH = 3421;    
    LUMBER_MILL = 3422;    
    GOLD_MINE = 3423;    
    STABLES = 3424;    
    CENARION_HOLD = 3425;    
    STAGHELM_POINT = 3426;    
    BRONZEBEARD_ENCAMPMENT = 3427;    
    AHN_QIRAJ = 3428;    
    RUINS_OF_AHN_QIRAJ0 = 3429;    
    TWILIGHTS_RUN = 3446;    
    ORTELLS_HIDEOUT = 3447;    
    SCARAB_TERRACE = 3448;    
    GENERALS_TERRACE = 3449;    
    THE_RESERVOIR = 3450;    
    THE_HATCHERY = 3451;    
    THE_COMB = 3452;    
    WATCHERS_TERRACE = 3453;    
    RUINS_OF_AHN_QIRAJ1 = 3454;    
    NAXXRAMAS = 3456;    
    CITY = 3459;    
    GATES_OF_AHN_QIRAJ = 3478;    
    RAVENHOLDT_MANOR = 3486;    
}

```