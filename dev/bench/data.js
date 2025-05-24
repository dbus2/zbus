window.BENCHMARK_DATA = {
  "lastUpdate": 1748109528993,
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
          "id": "dd39fa538a149898426e3cd7f026cd7b514fa9b6",
          "message": "Merge pull request #1384 from dbus2/renovate/uuid-1.x-lockfile\n\n⬆️ Update uuid to v1.17.0",
          "timestamp": "2025-05-23T13:16:03+02:00",
          "tree_id": "85920672760ebfab1c992a50e820ac8e76a3b0ea",
          "url": "https://github.com/dbus2/zbus/commit/dd39fa538a149898426e3cd7f026cd7b514fa9b6"
        },
        "date": 1747999649451,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2225,
            "range": "± 73",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3659933,
            "range": "± 36351",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 230,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3952315,
            "range": "± 73510",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 401,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 482,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 156,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 157,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 167,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 135,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 132,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 95,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 424261,
            "range": "± 4679",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 484591,
            "range": "± 2991",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1237495,
            "range": "± 7638",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2175168,
            "range": "± 88815",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2371703,
            "range": "± 34746",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4041467,
            "range": "± 169086",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1000929,
            "range": "± 18179",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312638,
            "range": "± 3018",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11468,
            "range": "± 373",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 86,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "29139614+renovate[bot]@users.noreply.github.com",
            "name": "renovate[bot]",
            "username": "renovate[bot]"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "3d7f80f677121948113fd507a43a0ea0a7d5a3f7",
          "message": "⬆️ micro: Update tokio to v1.45.1 (#1385)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [tokio](https://tokio.rs)\n([source](https://redirect.github.com/tokio-rs/tokio)) |\nworkspace.dependencies | patch | `1.45.0` -> `1.45.1` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>tokio-rs/tokio (tokio)</summary>\n\n###\n[`v1.45.1`](https://redirect.github.com/tokio-rs/tokio/releases/tag/tokio-1.45.1):\nTokio v1.45.1\n\n[Compare\nSource](https://redirect.github.com/tokio-rs/tokio/compare/tokio-1.45.0...tokio-1.45.1)\n\n### 1.45.1 (May 24th, 2025)\n\nThis fixes a regression on the wasm32-unknown-unknown target, where code\nthat previously did not panic due to calls to `Instant::now()` started\nfailing. This is due to the stabilization of the first time-based\nmetric.\n\n##### Fixed\n\n- Disable time-based metrics on wasm32-unknown-unknown ([#&#8203;7322])\n\n[#&#8203;7322]: https://redirect.github.com/tokio-rs/tokio/pull/7322\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC4xNi4wIiwidXBkYXRlZEluVmVyIjoiNDAuMTYuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-24T17:47:07Z",
          "tree_id": "47f8b38d92d54fbcfc5aac11ac7e4ae3f01d9cfe",
          "url": "https://github.com/dbus2/zbus/commit/3d7f80f677121948113fd507a43a0ea0a7d5a3f7"
        },
        "date": 1748109527320,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2326,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3599435,
            "range": "± 49664",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 241,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4235216,
            "range": "± 43449",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 495,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 23",
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
            "value": 167,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 135,
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
            "value": 94,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 419417,
            "range": "± 17956",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 529428,
            "range": "± 1517",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1182695,
            "range": "± 24138",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2288977,
            "range": "± 10010",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2306326,
            "range": "± 32844",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4080480,
            "range": "± 21473",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 969921,
            "range": "± 2451",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1289409,
            "range": "± 5209",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11203,
            "range": "± 181",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 89,
            "range": "± 1",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}