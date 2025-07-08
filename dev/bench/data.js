window.BENCHMARK_DATA = {
  "lastUpdate": 1751983962472,
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
          "id": "55c690159f2117636adb4ef4b56de9f28977808e",
          "message": "Merge pull request #1401 from A6GibKm/add-builder-for-flags\n\n✨ zb: Allow overriding some request name flags",
          "timestamp": "2025-07-08T16:01:18+02:00",
          "tree_id": "490e6ba6b8535ae6918e76ab95d8e5c7a894bd03",
          "url": "https://github.com/dbus2/zbus/commit/55c690159f2117636adb4ef4b56de9f28977808e"
        },
        "date": 1751983960737,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2220,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3505182,
            "range": "± 32406",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 257,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4462349,
            "range": "± 42981",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 400,
            "range": "± 5",
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
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 162,
            "range": "± 6",
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
            "value": 134,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 101,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 436965,
            "range": "± 22769",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 453097,
            "range": "± 751",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1136953,
            "range": "± 5957",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2141214,
            "range": "± 31866",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2233899,
            "range": "± 15223",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4071958,
            "range": "± 10052",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1031364,
            "range": "± 27372",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312750,
            "range": "± 6380",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11034,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}