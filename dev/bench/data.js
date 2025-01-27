window.BENCHMARK_DATA = {
  "lastUpdate": 1738011681339,
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
          "id": "9bccc739a93ea762b8c13e8abae2e2554442d43f",
          "message": "Merge pull request #1224 from zeenix/lifetime-fix\n\n🐛 zm: Drop unnecessary 'static lifetime requirements in proxy",
          "timestamp": "2025-01-23T16:44:51+01:00",
          "tree_id": "a542a59dbd65c023d3d5d2b5580e0185504b7d24",
          "url": "https://github.com/dbus2/zbus/commit/9bccc739a93ea762b8c13e8abae2e2554442d43f"
        },
        "date": 1737647768101,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2159,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2920770,
            "range": "± 13655",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 259,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3845446,
            "range": "± 16086",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 404,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 510,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 111,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 3",
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
            "value": 218674,
            "range": "± 9119",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412522,
            "range": "± 746",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 642793,
            "range": "± 5214",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2069720,
            "range": "± 37423",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1695412,
            "range": "± 26030",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4031035,
            "range": "± 6918",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167497,
            "range": "± 438",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1174353,
            "range": "± 2354",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10988,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 113,
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
          "id": "c3129816d6318de3673a9433a1393cc26a48b579",
          "message": "🔖 zb,zm: Release 5.3.1",
          "timestamp": "2025-01-23T16:47:48+01:00",
          "tree_id": "108147fae8fb926213c48ba699cecdd81061affd",
          "url": "https://github.com/dbus2/zbus/commit/c3129816d6318de3673a9433a1393cc26a48b579"
        },
        "date": 1737647973801,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2081,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2899007,
            "range": "± 37167",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 215,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4131950,
            "range": "± 14414",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 415,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 505,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 112,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 102,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 221182,
            "range": "± 1266",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412114,
            "range": "± 1091",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 646894,
            "range": "± 4011",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2077306,
            "range": "± 16675",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1704550,
            "range": "± 6479",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3956368,
            "range": "± 26544",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167089,
            "range": "± 981",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1126300,
            "range": "± 1526",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11060,
            "range": "± 378",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 112,
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
          "id": "3be463419c082ff9ad46993e58aff0ea89e040c8",
          "message": "Merge pull request #1225 from dbus2/renovate/rand-0.x\n\n⬆️ Update rand to 0.9.0",
          "timestamp": "2025-01-27T21:50:18+01:00",
          "tree_id": "ab48cee9c75f485af4ecd1fc2b127bc71d6a824d",
          "url": "https://github.com/dbus2/zbus/commit/3be463419c082ff9ad46993e58aff0ea89e040c8"
        },
        "date": 1738011679644,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2232,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3110502,
            "range": "± 37134",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 221,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3954749,
            "range": "± 18703",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 501,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 12",
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
            "value": 117,
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
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218826,
            "range": "± 3210",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411747,
            "range": "± 1931",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 636603,
            "range": "± 3537",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2088054,
            "range": "± 4369",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1704070,
            "range": "± 16777",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3880751,
            "range": "± 10346",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168181,
            "range": "± 588",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1142888,
            "range": "± 4188",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11009,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}