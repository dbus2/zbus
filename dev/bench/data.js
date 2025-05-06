window.BENCHMARK_DATA = {
  "lastUpdate": 1746530715988,
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
      },
      {
        "commit": {
          "author": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "committer": {
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "distinct": true,
          "id": "df4f651e2086323d4e59d2ee79ee5c616f942d3c",
          "message": "✏️  book: Fix two minor mistakes",
          "timestamp": "2025-05-03T23:28:51+02:00",
          "tree_id": "e1319c29b969454bf66c0dcb4bdc19ca30721bcb",
          "url": "https://github.com/dbus2/zbus/commit/df4f651e2086323d4e59d2ee79ee5c616f942d3c"
        },
        "date": 1746308438877,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2292,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3355575,
            "range": "± 18095",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 285,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3853460,
            "range": "± 36141",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 394,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 486,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 12",
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
            "value": 162,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
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
            "value": 367936,
            "range": "± 8092",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410392,
            "range": "± 2117",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 990255,
            "range": "± 4129",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2068957,
            "range": "± 7718",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2121482,
            "range": "± 24615",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4342599,
            "range": "± 14224",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 565896,
            "range": "± 2073",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1122137,
            "range": "± 2189",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11042,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
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
          "id": "0820baf62a8804e549f8ec0f5a43a2f4a2e95d55",
          "message": "Merge pull request #1351 from dbus2/renovate/nix-0.x\n\n⬆️ Update nix to 0.30",
          "timestamp": "2025-05-04T21:30:32+02:00",
          "tree_id": "7070ae9c8170b6ef0f50943f9e82ac78898154c1",
          "url": "https://github.com/dbus2/zbus/commit/0820baf62a8804e549f8ec0f5a43a2f4a2e95d55"
        },
        "date": 1746387710750,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2216,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3435777,
            "range": "± 25438",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 281,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3899888,
            "range": "± 33362",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 391,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 154,
            "range": "± 4",
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
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 138,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 77,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 354766,
            "range": "± 3269",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 437760,
            "range": "± 11931",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 940215,
            "range": "± 8165",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2070669,
            "range": "± 34499",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2051412,
            "range": "± 7729",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4129737,
            "range": "± 19574",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 595843,
            "range": "± 5089",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1116515,
            "range": "± 3048",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10910,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 118,
            "range": "± 2",
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
          "id": "1921fac13625803a7efc612dabe1ce831d70fe5e",
          "message": "Merge pull request #1362 from elmarco/fix\n\nWarning fixes",
          "timestamp": "2025-05-05T15:03:27+02:00",
          "tree_id": "609b985c822440f022fb491286914148b55297ec",
          "url": "https://github.com/dbus2/zbus/commit/1921fac13625803a7efc612dabe1ce831d70fe5e"
        },
        "date": 1746450878654,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2261,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3339577,
            "range": "± 42481",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 284,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3920847,
            "range": "± 76597",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 385,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 496,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 1",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 132,
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 354285,
            "range": "± 2838",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418261,
            "range": "± 579",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 951377,
            "range": "± 5113",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2070605,
            "range": "± 9452",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2073309,
            "range": "± 16171",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3971461,
            "range": "± 34417",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 598753,
            "range": "± 1688",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1128224,
            "range": "± 7085",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10904,
            "range": "± 84",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 102,
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
          "id": "4e5e67ab31c456e3e40edfd7ba897c9c77b5deb5",
          "message": "Merge pull request #1363 from dbus2/renovate/tokio-1.x-lockfile\n\n⬆️ Update tokio to v1.45.0",
          "timestamp": "2025-05-06T13:14:06+02:00",
          "tree_id": "cf4a4d28de943839d58b44681175257ea2b234e4",
          "url": "https://github.com/dbus2/zbus/commit/4e5e67ab31c456e3e40edfd7ba897c9c77b5deb5"
        },
        "date": 1746530714951,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2307,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3370676,
            "range": "± 29034",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 276,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3874111,
            "range": "± 13957",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 384,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 494,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 3",
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
            "value": 167,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 137,
            "range": "± 2",
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
            "value": 372581,
            "range": "± 1836",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411392,
            "range": "± 759",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1007966,
            "range": "± 5519",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2065519,
            "range": "± 12959",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2110693,
            "range": "± 10148",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4065391,
            "range": "± 19699",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 596013,
            "range": "± 2419",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1120278,
            "range": "± 3454",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10928,
            "range": "± 28",
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