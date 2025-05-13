window.BENCHMARK_DATA = {
  "lastUpdate": 1747174637143,
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
          "id": "bda99b56689ba29d5b01a2137804bbf6d4dac9b2",
          "message": "Merge pull request #1376 from zeenix/fix-serialize-dict-empty-struct\n\n🚑️ zd: Fix use of empty structs with SerializeDict",
          "timestamp": "2025-05-13T23:52:24+02:00",
          "tree_id": "eb4913c344927609a3051792f9e46ce90aad9eb0",
          "url": "https://github.com/dbus2/zbus/commit/bda99b56689ba29d5b01a2137804bbf6d4dac9b2"
        },
        "date": 1747173815636,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2232,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3323230,
            "range": "± 39488",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 272,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4013862,
            "range": "± 71811",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 167,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 135,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 135,
            "range": "± 1",
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
            "value": 344745,
            "range": "± 1322",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 421313,
            "range": "± 770",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 924952,
            "range": "± 5006",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2062143,
            "range": "± 10003",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2020923,
            "range": "± 5663",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4014650,
            "range": "± 8445",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595436,
            "range": "± 3986",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1120041,
            "range": "± 2152",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10910,
            "range": "± 44",
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
          "id": "b37180f1e3504673f7fecbe15ad0b0d707fb8b86",
          "message": "Merge pull request #1377 from zeenix/zv-release\n\n🔖 zv,zd: Release 5.5.3",
          "timestamp": "2025-05-14T00:06:04+02:00",
          "tree_id": "e077171d75ca70c1a278c40d3a470d630c8e7649",
          "url": "https://github.com/dbus2/zbus/commit/b37180f1e3504673f7fecbe15ad0b0d707fb8b86"
        },
        "date": 1747174635830,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2273,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3311065,
            "range": "± 23808",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 238,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3873541,
            "range": "± 15120",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 394,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 490,
            "range": "± 4",
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
            "value": 156,
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
            "value": 131,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 390018,
            "range": "± 1334",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410290,
            "range": "± 1579",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1036032,
            "range": "± 3327",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2070852,
            "range": "± 22704",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2136668,
            "range": "± 6270",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3797339,
            "range": "± 13581",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 659200,
            "range": "± 857",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1183228,
            "range": "± 6887",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10885,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}