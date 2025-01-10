window.BENCHMARK_DATA = {
  "lastUpdate": 1736527232345,
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
          "id": "0e37c6c6079898c46542f44b5b63747d2bb0786d",
          "message": "Merge pull request #1211 from zeenix/pr-template\n\n🚸 Make PR template a comment",
          "timestamp": "2025-01-10T17:13:37+01:00",
          "tree_id": "45de74b1235998faac1c160d4477220e1c582cb1",
          "url": "https://github.com/dbus2/zbus/commit/0e37c6c6079898c46542f44b5b63747d2bb0786d"
        },
        "date": 1736526295338,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2059,
            "range": "± 255",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2910104,
            "range": "± 22783",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 228,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3821222,
            "range": "± 14106",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 400,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 506,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 111,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218234,
            "range": "± 1048",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 413997,
            "range": "± 1219",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634240,
            "range": "± 2627",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2085401,
            "range": "± 9703",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1779866,
            "range": "± 4052",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3861482,
            "range": "± 20048",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167922,
            "range": "± 655",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1120313,
            "range": "± 2318",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10966,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 120,
            "range": "± 3",
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
          "id": "16b7f60e505e3a1a3ced253c21ae8140b22f4e70",
          "message": "Merge pull request #1212 from zeenix/inherit-from-workspace\n\n♻️  all: Inherit common Cargo.toml props from the workspace",
          "timestamp": "2025-01-10T17:29:24+01:00",
          "tree_id": "d11ba29d3316af5cc86dfa4ce9e23ad5c77753b0",
          "url": "https://github.com/dbus2/zbus/commit/16b7f60e505e3a1a3ced253c21ae8140b22f4e70"
        },
        "date": 1736527230352,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2137,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2864296,
            "range": "± 40042",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 214,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3791746,
            "range": "± 45242",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 414,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 498,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 107,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 113,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 219195,
            "range": "± 1486",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 417306,
            "range": "± 3470",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631181,
            "range": "± 8343",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2071353,
            "range": "± 11913",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1720883,
            "range": "± 28305",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3772205,
            "range": "± 22777",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 169866,
            "range": "± 1089",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1118824,
            "range": "± 19815",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10994,
            "range": "± 70",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 119,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}