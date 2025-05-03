window.BENCHMARK_DATA = {
  "lastUpdate": 1746307366784,
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
          "id": "384b9cb8fdebd76b0a5c33eac66793a962d2bb50",
          "message": "Merge pull request #1359 from zeenix/release-zbus-5.6.0\n\n🔖 zb,zm: Release 5.6.0",
          "timestamp": "2025-05-03T23:11:31+02:00",
          "tree_id": "4a79e69ff649caafb2dd0ecac104004e6377c56f",
          "url": "https://github.com/dbus2/zbus/commit/384b9cb8fdebd76b0a5c33eac66793a962d2bb50"
        },
        "date": 1746307365519,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2217,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3373288,
            "range": "± 19955",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 297,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3880788,
            "range": "± 28354",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 412,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 496,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 5",
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
            "value": 163,
            "range": "± 5",
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
            "value": 132,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 360988,
            "range": "± 3489",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 416668,
            "range": "± 1755",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 985822,
            "range": "± 5538",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2083149,
            "range": "± 9356",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2055733,
            "range": "± 10602",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4013537,
            "range": "± 10797",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 564264,
            "range": "± 855",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1128342,
            "range": "± 34848",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11047,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 116,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}