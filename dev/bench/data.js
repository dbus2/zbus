window.BENCHMARK_DATA = {
  "lastUpdate": 1730822025872,
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
      }
    ]
  }
}