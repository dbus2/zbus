window.BENCHMARK_DATA = {
  "lastUpdate": 1730981420841,
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
          "id": "6101e80dd37dbc33119ed7df0698d9c9af93a12f",
          "message": "Merge pull request #1124 from zeenix/benchmarks-in-ci\n\n👷 CI: Run benchmarks as part of the CI on pushes to main",
          "timestamp": "2024-11-05T16:42:23+01:00",
          "tree_id": "e84c41515c21ae8a1ea9dfdd9b22bf5a32a66f8a",
          "url": "https://github.com/dbus2/zbus/commit/6101e80dd37dbc33119ed7df0698d9c9af93a12f"
        },
        "date": 1730822024694,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2218,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2963730,
            "range": "± 55139",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 218,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4357543,
            "range": "± 12227",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 517,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 3",
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
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 214700,
            "range": "± 1165",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411511,
            "range": "± 878",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 627448,
            "range": "± 1813",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2088631,
            "range": "± 12792",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1774731,
            "range": "± 6479",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4022043,
            "range": "± 32241",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166151,
            "range": "± 436",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1138185,
            "range": "± 2109",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11192,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 130,
            "range": "± 0",
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
          "id": "22e772f8f42068fe6fb98dfabdcd4a191143bfb0",
          "message": "Merge pull request #1126 from zeenix/async-process-dep\n\n➖ zb: Tie async-process dep to async-io feature",
          "timestamp": "2024-11-05T17:12:54+01:00",
          "tree_id": "3e23604e5080b09226398f1b2acd315c6b73d2b1",
          "url": "https://github.com/dbus2/zbus/commit/22e772f8f42068fe6fb98dfabdcd4a191143bfb0"
        },
        "date": 1730823856177,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2158,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2896931,
            "range": "± 23061",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3835493,
            "range": "± 6559",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 414,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 514,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 5",
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
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 214745,
            "range": "± 3174",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412539,
            "range": "± 1159",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 628246,
            "range": "± 2623",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2099173,
            "range": "± 11865",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1739449,
            "range": "± 4936",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3958390,
            "range": "± 40029",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166073,
            "range": "± 1376",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1139326,
            "range": "± 6311",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10952,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 130,
            "range": "± 0",
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
          "id": "40ea8c1ba27ef23460d604c53715163d3faa3798",
          "message": "Merge pull request #1127 from zeenix/fix-bench-ci\n\n💚 CI: Bump benchmarks alert threshold to 200% & don't fail on alert",
          "timestamp": "2024-11-05T17:43:09+01:00",
          "tree_id": "3a7425ab0054cbe8bc2a7c74284a18d93ba83539",
          "url": "https://github.com/dbus2/zbus/commit/40ea8c1ba27ef23460d604c53715163d3faa3798"
        },
        "date": 1730825661952,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2184,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3105242,
            "range": "± 132974",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3841816,
            "range": "± 9174",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 415,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 519,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 4",
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
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 214599,
            "range": "± 1417",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 414159,
            "range": "± 1196",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 625844,
            "range": "± 31510",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2159827,
            "range": "± 19364",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1723985,
            "range": "± 3695",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4020931,
            "range": "± 12576",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166829,
            "range": "± 310",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1139424,
            "range": "± 1527",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10933,
            "range": "± 155",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 132,
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
          "id": "538c9742973c0a8345a5cbd641ee17d6b5cac0b2",
          "message": "Merge pull request #1128 from zeenix/split-method\n\n♻️  zb: Split a big internal method a bit",
          "timestamp": "2024-11-05T23:28:57+01:00",
          "tree_id": "09bf456b808ad263d8d7ca62b3e5d061e554de4b",
          "url": "https://github.com/dbus2/zbus/commit/538c9742973c0a8345a5cbd641ee17d6b5cac0b2"
        },
        "date": 1730846427874,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2178,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3061867,
            "range": "± 224245",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 226,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4081372,
            "range": "± 50515",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 519,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 107,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 109,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 219539,
            "range": "± 1942",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 415389,
            "range": "± 2034",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 635383,
            "range": "± 2658",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2106153,
            "range": "± 11845",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1707060,
            "range": "± 8541",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4067296,
            "range": "± 39038",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166157,
            "range": "± 497",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1179708,
            "range": "± 6961",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11138,
            "range": "± 77",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 131,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "distinct": true,
          "id": "3aac8f40e3204f3cf566586c3efdbf2ce58af9d2",
          "message": "⬆️ micro: Update tokio to v1.41.1",
          "timestamp": "2024-11-07T11:58:57Z",
          "tree_id": "c2525337d940f41e6d3b08778bd1f28c6b5537ac",
          "url": "https://github.com/dbus2/zbus/commit/3aac8f40e3204f3cf566586c3efdbf2ce58af9d2"
        },
        "date": 1730981419889,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2118,
            "range": "± 53",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3021022,
            "range": "± 14703",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 211,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3886691,
            "range": "± 22695",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 514,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 106,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 116,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 217714,
            "range": "± 2938",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423239,
            "range": "± 1394",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631525,
            "range": "± 1255",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2137332,
            "range": "± 6861",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1726750,
            "range": "± 4983",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3922943,
            "range": "± 26255",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166523,
            "range": "± 777",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131008,
            "range": "± 7252",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11512,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}