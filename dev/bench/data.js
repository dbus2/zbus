window.BENCHMARK_DATA = {
  "lastUpdate": 1737647769128,
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
      }
    ]
  }
}