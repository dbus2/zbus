window.BENCHMARK_DATA = {
  "lastUpdate": 1730823857154,
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
      }
    ]
  }
}