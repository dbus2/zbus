window.BENCHMARK_DATA = {
  "lastUpdate": 1738881283306,
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
          "id": "d60b895ae438f43db95b64a66714b5c7c37d9134",
          "message": "Merge pull request #1247 from jplatte/futures-lite\n\nzb: Replace futures-util runtime dependency with futures-lite",
          "timestamp": "2025-02-06T23:23:23+01:00",
          "tree_id": "18cce99d170279fb8921d7b9bc97b97d2996384b",
          "url": "https://github.com/dbus2/zbus/commit/d60b895ae438f43db95b64a66714b5c7c37d9134"
        },
        "date": 1738881282144,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2230,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3145050,
            "range": "± 28525",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 281,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4303520,
            "range": "± 16804",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 425,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 494,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 3",
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
            "value": 166,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 133,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 130,
            "range": "± 1",
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
            "value": 217077,
            "range": "± 1446",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410821,
            "range": "± 4089",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 638111,
            "range": "± 2622",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2092965,
            "range": "± 11218",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1821607,
            "range": "± 43536",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4025122,
            "range": "± 15419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166718,
            "range": "± 1579",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1130037,
            "range": "± 2586",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11109,
            "range": "± 36",
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