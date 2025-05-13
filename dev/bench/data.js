window.BENCHMARK_DATA = {
  "lastUpdate": 1747173816725,
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
          "id": "bda99b56689ba29d5b01a2137804bbf6d4dac9b2",
          "message": "Merge pull request #1376 from zeenix/fix-serialize-dict-empty-struct\n\n🚑️ zd: Fix use of empty structs with SerializeDict",
          "timestamp": "2025-05-13T23:52:24+02:00",
          "tree_id": "eb4913c344927609a3051792f9e46ce90aad9eb0",
          "url": "https://github.com/dbus2/zbus/commit/bda99b56689ba29d5b01a2137804bbf6d4dac9b2"
        },
        "date": 1747173815636,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2232,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3323230,
            "range": "± 39488",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 272,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4013862,
            "range": "± 71811",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 167,
            "range": "± 2",
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
            "value": 135,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 344745,
            "range": "± 1322",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 421313,
            "range": "± 770",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 924952,
            "range": "± 5006",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2062143,
            "range": "± 10003",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2020923,
            "range": "± 5663",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4014650,
            "range": "± 8445",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595436,
            "range": "± 3986",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1120041,
            "range": "± 2152",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10910,
            "range": "± 44",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}