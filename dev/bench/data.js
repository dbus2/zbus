window.BENCHMARK_DATA = {
  "lastUpdate": 1738947285417,
  "repoUrl": "https://github.com/dbus2/zbus",
  "entries": {
    "Benchmark": [
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "d60b895ae438f43db95b64a66714b5c7c37d9134",
          "message": "Merge pull request #1247 from jplatte/futures-lite\n\nzb: Replace futures-util runtime dependency with futures-lite",
          "timestamp": "2025-02-06T23:23:23+01:00",
          "tree_id": "18cce99d170279fb8921d7b9bc97b97d2996384b",
          "url": "https://github.com/dbus2/zbus/commit/d60b895ae438f43db95b64a66714b5c7c37d9134"
        },
        "date": 1738881282144,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2230,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3145050,
            "range": "± 28525",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 281,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4303520,
            "range": "± 16804",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 425,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 494,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 166,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 133,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 217077,
            "range": "± 1446",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410821,
            "range": "± 4089",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 638111,
            "range": "± 2622",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2092965,
            "range": "± 11218",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1821607,
            "range": "± 43536",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4025122,
            "range": "± 15419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166718,
            "range": "± 1579",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1130037,
            "range": "± 2586",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11109,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "17576896546b1da80681d2d622a79db59eb019b8",
          "message": "Merge pull request #1250 from zeenix/dict-as-prop\n\n✨ zv: *Value derive now supports optional fields in dict structs",
          "timestamp": "2025-02-06T23:27:20+01:00",
          "tree_id": "2a5d04f441ffc41c1dd92f2166258cc0f381b4e8",
          "url": "https://github.com/dbus2/zbus/commit/17576896546b1da80681d2d622a79db59eb019b8"
        },
        "date": 1738881520326,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2245,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3140942,
            "range": "± 27139",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 262,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4017056,
            "range": "± 27883",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 422,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 500,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 165,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 132,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 217476,
            "range": "± 1442",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 408788,
            "range": "± 3025",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 647663,
            "range": "± 3116",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2095585,
            "range": "± 26091",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1808560,
            "range": "± 6767",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4075703,
            "range": "± 6028",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167103,
            "range": "± 259",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1129314,
            "range": "± 4382",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11033,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "8a6cedc0cc18e18686e4f0f6400f99afc7922851",
          "message": "📝 README.md: Tiny fix",
          "timestamp": "2025-02-07T11:19:03+01:00",
          "tree_id": "321759ca45d7051b8978e6f7111ef2a8dd269137",
          "url": "https://github.com/dbus2/zbus/commit/8a6cedc0cc18e18686e4f0f6400f99afc7922851"
        },
        "date": 1738924218213,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2273,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3194749,
            "range": "± 31273",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 252,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4018980,
            "range": "± 12181",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 414,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 485,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 134,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 219334,
            "range": "± 2860",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 414526,
            "range": "± 2572",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631941,
            "range": "± 3045",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2086021,
            "range": "± 10060",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1920111,
            "range": "± 6584",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3868936,
            "range": "± 8198",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167418,
            "range": "± 501",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1126235,
            "range": "± 5398",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11037,
            "range": "± 72",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "distinct": true,
          "id": "8fff8e2d40d8400cf0e27eef80f0f3945c91a7e3",
          "message": "🔖 zv,zd: Release 5.4.0",
          "timestamp": "2025-02-07T14:40:22+01:00",
          "tree_id": "1f2e5766e32d3fbb930aa88d033e2dca5aa72b3a",
          "url": "https://github.com/dbus2/zbus/commit/8fff8e2d40d8400cf0e27eef80f0f3945c91a7e3"
        },
        "date": 1738936369218,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2291,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3572831,
            "range": "± 26937",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 258,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4295244,
            "range": "± 18122",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 411,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 482,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 157,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 131,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 215219,
            "range": "± 1423",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 430563,
            "range": "± 1330",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 637803,
            "range": "± 2182",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2097593,
            "range": "± 9168",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1838908,
            "range": "± 24736",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3885956,
            "range": "± 30979",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167866,
            "range": "± 297",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1126428,
            "range": "± 1439",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10862,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dbc1b9b0bcc3ca29427bc7837dbbbf43fb30a6a7",
          "message": "Merge pull request #1251 from zeenix/zb-release\n\n🔖 zb,zm: Release 5.5.0",
          "timestamp": "2025-02-07T17:43:17+01:00",
          "tree_id": "d1f0a4fb6d4e80460e99559263837f7fef873e8e",
          "url": "https://github.com/dbus2/zbus/commit/dbc1b9b0bcc3ca29427bc7837dbbbf43fb30a6a7"
        },
        "date": 1738947283792,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2369,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3141915,
            "range": "± 32103",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 244,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4255825,
            "range": "± 21889",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 419,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 496,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 158,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 128,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 128,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 217359,
            "range": "± 1811",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 424321,
            "range": "± 8245",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634889,
            "range": "± 6360",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2130471,
            "range": "± 20038",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1798572,
            "range": "± 9533",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3826497,
            "range": "± 19941",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167727,
            "range": "± 2079",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1468882,
            "range": "± 6371",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11032,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}