window.BENCHMARK_DATA = {
  "lastUpdate": 1747768059711,
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
          "id": "814fda4d14352e81a927d31587f12810f89f76cf",
          "message": "Merge pull request #1366 from KmolYuan/fix-1312\n\n✨ zm: Copy attributes to `receive_*_changed` and `cached_*` methods",
          "timestamp": "2025-05-20T20:56:01+02:00",
          "tree_id": "5c8bd3cca5478676298d940a569858655af7cf5e",
          "url": "https://github.com/dbus2/zbus/commit/814fda4d14352e81a927d31587f12810f89f76cf"
        },
        "date": 1747768058318,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2254,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3669790,
            "range": "± 30496",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 221,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4159947,
            "range": "± 40443",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 449,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 503,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 158,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 156,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 169,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 134,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 456060,
            "range": "± 14504",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 669824,
            "range": "± 1066",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1156672,
            "range": "± 5336",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2185216,
            "range": "± 12481",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2282212,
            "range": "± 14650",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4087544,
            "range": "± 13729",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1000620,
            "range": "± 4679",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1272766,
            "range": "± 2326",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11034,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 93,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}