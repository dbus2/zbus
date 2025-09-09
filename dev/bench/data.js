window.BENCHMARK_DATA = {
  "lastUpdate": 1757409256494,
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
          "id": "edd9a3c3d3f7fc2520cd2c1f07b27ed5f2245a21",
          "message": "Merge pull request #1494 from zeenix/prep-zb-5.11\n\n🔖 zb,zm: Release 5.11.0",
          "timestamp": "2025-09-09T11:02:57+02:00",
          "tree_id": "0ae19bade94a30a1275ec06ff84f93e706974244",
          "url": "https://github.com/dbus2/zbus/commit/edd9a3c3d3f7fc2520cd2c1f07b27ed5f2245a21"
        },
        "date": 1757409255364,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2133,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3184280,
            "range": "± 32349",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 240,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4108947,
            "range": "± 33130",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 436,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 520,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 97,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 160,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 130,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 129,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 250150,
            "range": "± 1189",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454740,
            "range": "± 2664",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 723511,
            "range": "± 4161",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2209665,
            "range": "± 6880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1806343,
            "range": "± 3864",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4464724,
            "range": "± 39602",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 538219,
            "range": "± 1284",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1254792,
            "range": "± 2798",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10883,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 97,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}