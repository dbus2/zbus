window.BENCHMARK_DATA = {
  "lastUpdate": 1751660674408,
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
          "id": "3d3f356b06f2f95a7b28a49e590f354e0368742d",
          "message": "⬆️ micro: Update async-io to v2.4.1 (#1386)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [async-io](https://redirect.github.com/smol-rs/async-io) |\nworkspace.dependencies | patch | `2.4.0` -> `2.4.1` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>smol-rs/async-io (async-io)</summary>\n\n###\n[`v2.4.1`](https://redirect.github.com/smol-rs/async-io/blob/HEAD/CHANGELOG.md#Version-241)\n\n[Compare\nSource](https://redirect.github.com/smol-rs/async-io/compare/v2.4.0...v2.4.1)\n\n- Update to rustix version 1.0.7.\n([#&#8203;221](https://redirect.github.com/smol-rs/async-io/issues/221))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC4xNi4wIiwidXBkYXRlZEluVmVyIjoiNDAuMTYuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-25T18:54:27Z",
          "tree_id": "86b32a4a5e33495334c9b983525e62a689298c46",
          "url": "https://github.com/dbus2/zbus/commit/3d3f356b06f2f95a7b28a49e590f354e0368742d"
        },
        "date": 1748199958418,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2279,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3676772,
            "range": "± 131694",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 250,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3927677,
            "range": "± 34442",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 411,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 493,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 157,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 166,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 137,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 448918,
            "range": "± 29518",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 458380,
            "range": "± 745",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1145944,
            "range": "± 6002",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2114985,
            "range": "± 32385",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2225427,
            "range": "± 10604",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4213808,
            "range": "± 23369",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1000244,
            "range": "± 2296",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312407,
            "range": "± 6356",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11402,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 82,
            "range": "± 1",
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
          "id": "31ad20b8ee4e47be1b0386cc9fa6bbf2b443f0df",
          "message": "⬆️ micro: Update clap to v4.5.39 (#1388)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) |\nworkspace.dependencies | patch | `4.5.38` -> `4.5.39` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.39`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4539---2025-05-27)\n\n[Compare\nSource](https://redirect.github.com/clap-rs/clap/compare/v4.5.38...v4.5.39)\n\n##### Fixes\n\n-   *(help)* Show short flag aliases before long\n-   *(help)* Merge the short and long flag alias lists\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC4xNi4wIiwidXBkYXRlZEluVmVyIjoiNDAuMTYuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-05-27T23:12:35Z",
          "tree_id": "12503cd06b89b95d854ffa8d453587baa218c4e9",
          "url": "https://github.com/dbus2/zbus/commit/31ad20b8ee4e47be1b0386cc9fa6bbf2b443f0df"
        },
        "date": 1748388238959,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2235,
            "range": "± 108",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3626566,
            "range": "± 44491",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 232,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4147443,
            "range": "± 9266",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 482,
            "range": "± 2",
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
            "range": "± 18",
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
            "value": 94,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 375084,
            "range": "± 7910",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 468674,
            "range": "± 461",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1128136,
            "range": "± 29657",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2181562,
            "range": "± 10700",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2223415,
            "range": "± 8068",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4140243,
            "range": "± 12311",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 907758,
            "range": "± 4588",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1281501,
            "range": "± 2629",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11132,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 84,
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
          "id": "5609e3f937b215936f1f15d4fe0f65bb5b1ea8d1",
          "message": "Merge pull request #1391 from wezm/ppc\n\n🐛 zv: Fix build on platforms without 64-bit atomics",
          "timestamp": "2025-06-02T12:25:15+02:00",
          "tree_id": "1892ad207332e2b923ec221c43887ee56d2098a7",
          "url": "https://github.com/dbus2/zbus/commit/5609e3f937b215936f1f15d4fe0f65bb5b1ea8d1"
        },
        "date": 1748860602422,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2245,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3668167,
            "range": "± 38158",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 243,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3926258,
            "range": "± 18643",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 489,
            "range": "± 5",
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
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 136,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 137,
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
            "value": 389235,
            "range": "± 3953",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 455611,
            "range": "± 1009",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1101874,
            "range": "± 5848",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2145047,
            "range": "± 13416",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2250925,
            "range": "± 3744",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4091283,
            "range": "± 106761",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 908163,
            "range": "± 12773",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1258026,
            "range": "± 7427",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10840,
            "range": "± 88",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "75f52f103b235024665ad6042676859364ca00e4",
          "message": "Merge pull request #1392 from zeenix/drop-deprecated-function\n\n👽️ zb,zv,zn: Use `std::hint::black_box` in benchmarks code",
          "timestamp": "2025-06-02T22:32:56+02:00",
          "tree_id": "8ad33453a8a8d4ad68d399e40747eb0635d3054d",
          "url": "https://github.com/dbus2/zbus/commit/75f52f103b235024665ad6042676859364ca00e4"
        },
        "date": 1748897066983,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2289,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3569633,
            "range": "± 58605",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 239,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4204829,
            "range": "± 36388",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 452,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 528,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 156,
            "range": "± 8",
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
            "value": 136,
            "range": "± 8",
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
            "value": 427729,
            "range": "± 26592",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454102,
            "range": "± 3891",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1145228,
            "range": "± 6898",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2135889,
            "range": "± 10326",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2235195,
            "range": "± 7718",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4078956,
            "range": "± 12913",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 907210,
            "range": "± 2572",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1310737,
            "range": "± 1601",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10930,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
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
          "id": "60b2143cd1735df04552faaf07f4005b3256acc6",
          "message": "⬆️ micro: Update camino to v1.1.10 (#1393)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [camino](https://redirect.github.com/camino-rs/camino) |\nworkspace.dependencies | patch | `1.1.9` -> `1.1.10` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>camino-rs/camino (camino)</summary>\n\n###\n[`v1.1.10`](https://redirect.github.com/camino-rs/camino/blob/HEAD/CHANGELOG.md#1110---2025-06-02)\n\n[Compare\nSource](https://redirect.github.com/camino-rs/camino/compare/camino-1.1.9...camino-1.1.10)\n\n##### Changed\n\n- Hand-write serde implementations, dropping the dependency on\n`serde_derive`. Thanks to [Enselic](https://redirect.github.com/Enselic)\nfor initiating the discussion and for your first contribution!\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC4zMy42IiwidXBkYXRlZEluVmVyIjoiNDAuMzMuNiIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-02T20:41:57Z",
          "tree_id": "4d3f8b7999affb1a5002ce8895a4b6a73b57ebc3",
          "url": "https://github.com/dbus2/zbus/commit/60b2143cd1735df04552faaf07f4005b3256acc6"
        },
        "date": 1748897614619,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2287,
            "range": "± 167",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3659708,
            "range": "± 22265",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 248,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4115180,
            "range": "± 7639",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 408,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 494,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 5",
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
            "value": 136,
            "range": "± 1",
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
            "value": 509075,
            "range": "± 10968",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 455368,
            "range": "± 1888",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1149840,
            "range": "± 6339",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2134250,
            "range": "± 16720",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2245032,
            "range": "± 3546",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4043821,
            "range": "± 14521",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 938675,
            "range": "± 14961",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1313286,
            "range": "± 1349",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10811,
            "range": "± 71",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
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
          "id": "22182f00da8d7d29508aa9db58fd90fd583bd56f",
          "message": "Merge pull request #1399 from zeenix/add-claude-md\n\n🤖 Add CLAUDE.md",
          "timestamp": "2025-06-06T16:26:20+02:00",
          "tree_id": "f0bf4b55922c498821ad673b6aa49da2a0b7a3cc",
          "url": "https://github.com/dbus2/zbus/commit/22182f00da8d7d29508aa9db58fd90fd583bd56f"
        },
        "date": 1749220675523,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2235,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3651758,
            "range": "± 42122",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 262,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3927154,
            "range": "± 94883",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 503,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 155,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 166,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 135,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
            "range": "± 10",
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
            "value": 427291,
            "range": "± 25400",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 459772,
            "range": "± 4126",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1138310,
            "range": "± 15714",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2278941,
            "range": "± 25609",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2267188,
            "range": "± 32035",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4317904,
            "range": "± 75341",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 937853,
            "range": "± 19188",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1258721,
            "range": "± 13769",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11011,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
            "range": "± 4",
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
          "id": "e9b35f6eb160f12eaa007f994043cd8626b47003",
          "message": "⬆️ micro: Update enumflags2 to v0.7.12 (#1403)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [enumflags2](https://redirect.github.com/meithecatte/enumflags2) |\nworkspace.dependencies | patch | `0.7.11` -> `0.7.12` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>meithecatte/enumflags2 (enumflags2)</summary>\n\n###\n[`v0.7.12`](https://redirect.github.com/meithecatte/enumflags2/compare/v0.7.11...v0.7.12)\n\n[Compare\nSource](https://redirect.github.com/meithecatte/enumflags2/compare/v0.7.11...v0.7.12)\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC40OC41IiwidXBkYXRlZEluVmVyIjoiNDAuNDguNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-09T23:03:10Z",
          "tree_id": "6a4a956fdcc795a0a40249434d12d40195c98aed",
          "url": "https://github.com/dbus2/zbus/commit/e9b35f6eb160f12eaa007f994043cd8626b47003"
        },
        "date": 1749510880201,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2271,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3482576,
            "range": "± 31849",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 231,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4202405,
            "range": "± 5511",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 395,
            "range": "± 3",
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
            "value": 156,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 157,
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
            "value": 136,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 426551,
            "range": "± 3166",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454571,
            "range": "± 1939",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1161094,
            "range": "± 3997",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2122065,
            "range": "± 4479",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2281263,
            "range": "± 8854",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4457136,
            "range": "± 18230",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1032154,
            "range": "± 2823",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1375551,
            "range": "± 7907",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10976,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "ddd8f37b1f94d164fe16929dfe2cec4b9ad52edd",
          "message": "⬆️ micro: Update clap to v4.5.40 (#1402)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) |\nworkspace.dependencies | patch | `4.5.39` -> `4.5.40` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.40`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4540---2025-06-09)\n\n[Compare\nSource](https://redirect.github.com/clap-rs/clap/compare/v4.5.39...v4.5.40)\n\n##### Features\n\n- Support quoted ids in `arg!()` macro (e.g. `arg!(\"check-config\":\n...)`)\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC40OC41IiwidXBkYXRlZEluVmVyIjoiNDAuNDguNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-09T23:03:40Z",
          "tree_id": "7f496e3023ea3fbfe0f1b042f66f0c688aff7c79",
          "url": "https://github.com/dbus2/zbus/commit/ddd8f37b1f94d164fe16929dfe2cec4b9ad52edd"
        },
        "date": 1749510904801,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2251,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3581427,
            "range": "± 23394",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 261,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3955180,
            "range": "± 9937",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 408,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 557,
            "range": "± 18",
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
            "value": 155,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
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
            "range": "± 0",
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
            "value": 469142,
            "range": "± 25643",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 451689,
            "range": "± 1437",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1173380,
            "range": "± 18986",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2150598,
            "range": "± 5664",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2330119,
            "range": "± 10806",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4231038,
            "range": "± 50158",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1002831,
            "range": "± 1289",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1251926,
            "range": "± 3041",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11143,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 131,
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
          "id": "e6ce851350b935c483d42437b41973c2c78fd8e2",
          "message": "⬆️ micro: Update syn to v2.0.102 (#1404)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) |\nworkspace.dependencies | patch | `2.0.101` -> `2.0.102` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.102`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.102)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.101...2.0.102)\n\n- Fix printing of nested Expr::Index and Expr::Tuple in non-full mode\n([#&#8203;1869](https://redirect.github.com/dtolnay/syn/issues/1869))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC40OC41IiwidXBkYXRlZEluVmVyIjoiNDAuNDguNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-10T06:03:54Z",
          "tree_id": "0e3aafe175f134437bb78d0e560cf69a2bf28dee",
          "url": "https://github.com/dbus2/zbus/commit/e6ce851350b935c483d42437b41973c2c78fd8e2"
        },
        "date": 1749536122355,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2176,
            "range": "± 35",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3406058,
            "range": "± 31680",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4176270,
            "range": "± 16136",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 497,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 4",
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
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 132,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 133,
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
            "value": 432199,
            "range": "± 21682",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 460734,
            "range": "± 970",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1165637,
            "range": "± 6599",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2151839,
            "range": "± 4799",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2267181,
            "range": "± 11764",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4067872,
            "range": "± 14023",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 969009,
            "range": "± 1293",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1259217,
            "range": "± 3358",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10892,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 132,
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
          "id": "2a2d6b6fb60e7fd59d91c8d8a329e59d58f1ea7e",
          "message": "⬆️ micro: Update winnow to v0.7.11 (#1405)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [winnow](https://redirect.github.com/winnow-rs/winnow) |\nworkspace.dependencies | patch | `0.7.10` -> `0.7.11` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>winnow-rs/winnow (winnow)</summary>\n\n###\n[`v0.7.11`](https://redirect.github.com/winnow-rs/winnow/blob/HEAD/CHANGELOG.md#0711---2025-06-10)\n\n[Compare\nSource](https://redirect.github.com/winnow-rs/winnow/compare/v0.7.10...v0.7.11)\n\n##### Fixes\n\n- Remove a stackoverflow in `PartialEq` and `PartialOrd` with `Bytes`\nand `BStr`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC40OC41IiwidXBkYXRlZEluVmVyIjoiNDAuNDguNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-10T17:12:03Z",
          "tree_id": "33bb79ce229a38677dbbb73f3e39243f41845b73",
          "url": "https://github.com/dbus2/zbus/commit/2a2d6b6fb60e7fd59d91c8d8a329e59d58f1ea7e"
        },
        "date": 1749576206988,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2193,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3416747,
            "range": "± 103561",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4134740,
            "range": "± 65174",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 507,
            "range": "± 3",
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
            "value": 153,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 166,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 136,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 135,
            "range": "± 5",
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
            "value": 418745,
            "range": "± 28775",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 466461,
            "range": "± 2229",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1140199,
            "range": "± 25078",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2168601,
            "range": "± 2617",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2258373,
            "range": "± 8414",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4025487,
            "range": "± 450704",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1001162,
            "range": "± 17325",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1312223,
            "range": "± 1264",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11031,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "5d148782d895af18d59faffe75f6bccfa1360dfc",
          "message": "Merge pull request #1406 from duelafn/typo\n\n📝 zn: doc typo",
          "timestamp": "2025-06-12T10:23:28+02:00",
          "tree_id": "e5238530771ab2725e7b9bcc693724f9a4011ad3",
          "url": "https://github.com/dbus2/zbus/commit/5d148782d895af18d59faffe75f6bccfa1360dfc"
        },
        "date": 1749717286323,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2187,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3490952,
            "range": "± 22934",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3991217,
            "range": "± 11613",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 400,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 499,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 155,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 157,
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
            "value": 136,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 135,
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
            "value": 399272,
            "range": "± 24090",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 446137,
            "range": "± 3527",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1181081,
            "range": "± 3035",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2173041,
            "range": "± 21595",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2280898,
            "range": "± 4521",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3940988,
            "range": "± 6400",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 937139,
            "range": "± 2128",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1261090,
            "range": "± 3342",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11005,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 131,
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
          "id": "0fa2aa6cd7b3302941c34669b18cb1b1b079c36a",
          "message": "⬆️ micro: Update syn to v2.0.103 (#1409)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) |\nworkspace.dependencies | patch | `2.0.102` -> `2.0.103` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.103`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.103)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.102...2.0.103)\n\n- Insert parentheses around binary operation with attribute\n([#&#8203;1871](https://redirect.github.com/dtolnay/syn/issues/1871))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC41MC4wIiwidXBkYXRlZEluVmVyIjoiNDAuNTAuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-13T04:58:32Z",
          "tree_id": "f745ed0a5b19f49fed0b0e68aac9a4b43a2cccef",
          "url": "https://github.com/dbus2/zbus/commit/0fa2aa6cd7b3302941c34669b18cb1b1b079c36a"
        },
        "date": 1749791396601,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2268,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3540780,
            "range": "± 38908",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 248,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3966193,
            "range": "± 10021",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 153,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 0",
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
            "range": "± 7",
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
            "value": 426568,
            "range": "± 28262",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 452607,
            "range": "± 687",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1141762,
            "range": "± 4774",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2111924,
            "range": "± 2870",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2253415,
            "range": "± 8291",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4290268,
            "range": "± 13725",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 907772,
            "range": "± 2470",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1263752,
            "range": "± 3325",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10943,
            "range": "± 66",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "c805e10ae4e8dfd1dbff872cd60c855beef857fd",
          "message": "⬆️ micro: Update syn to v2.0.104 (#1411)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) |\nworkspace.dependencies | patch | `2.0.103` -> `2.0.104` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.104`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.104)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.103...2.0.104)\n\n- Disallow attributes on range expression\n([#&#8203;1872](https://redirect.github.com/dtolnay/syn/issues/1872))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC42MC4xIiwidXBkYXRlZEluVmVyIjoiNDAuNjAuMSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-21T13:49:43Z",
          "tree_id": "8a13564f127843e9075a5f82bf038380d8a79726",
          "url": "https://github.com/dbus2/zbus/commit/c805e10ae4e8dfd1dbff872cd60c855beef857fd"
        },
        "date": 1750514464996,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2312,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3465993,
            "range": "± 28472",
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
            "value": 4067705,
            "range": "± 47220",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 486,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 158,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 157,
            "range": "± 2",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
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
            "value": 510466,
            "range": "± 3215",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454889,
            "range": "± 2015",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1143152,
            "range": "± 5492",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2121751,
            "range": "± 5248",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2207738,
            "range": "± 10481",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3971623,
            "range": "± 9618",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 968078,
            "range": "± 41870",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1301303,
            "range": "± 15302",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10915,
            "range": "± 56",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "7c749b5ff8607be7810c975c20901fe04024264c",
          "message": "Merge pull request #1408 from KmolYuan/zm-interface-cfgs\n\n✨zm: Support write-only properties",
          "timestamp": "2025-06-22T14:56:56+01:00",
          "tree_id": "f043a7fec68c4393c2448ab1e005fcc4542ac45c",
          "url": "https://github.com/dbus2/zbus/commit/7c749b5ff8607be7810c975c20901fe04024264c"
        },
        "date": 1750601305224,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2242,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3556402,
            "range": "± 30425",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 243,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4001983,
            "range": "± 29718",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 392,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
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
            "value": 162,
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
            "range": "± 2",
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
            "value": 410989,
            "range": "± 10625",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 454352,
            "range": "± 639",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1129727,
            "range": "± 6659",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2124381,
            "range": "± 6560",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2276497,
            "range": "± 10745",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4538276,
            "range": "± 60264",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 969336,
            "range": "± 1693",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1281471,
            "range": "± 9863",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11128,
            "range": "± 879",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
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
          "id": "02563f5a991987fe730f4f9ba0bf4fb3714f1c8d",
          "message": "Merge pull request #1416 from dbus2/clippy-fixes\n\n🚨 all: Fix against latest clippy",
          "timestamp": "2025-06-30T13:46:39+02:00",
          "tree_id": "bf56f814c6ff634cf00a832b16e001c5f0d12526",
          "url": "https://github.com/dbus2/zbus/commit/02563f5a991987fe730f4f9ba0bf4fb3714f1c8d"
        },
        "date": 1751284676906,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2259,
            "range": "± 151",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3474159,
            "range": "± 42497",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 234,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4022040,
            "range": "± 22353",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 384,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 509,
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
            "value": 136,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 76,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 415847,
            "range": "± 18264",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 460681,
            "range": "± 8160",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1189867,
            "range": "± 13671",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2131814,
            "range": "± 17672",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2285204,
            "range": "± 64577",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4227213,
            "range": "± 32962",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1001706,
            "range": "± 4183",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1375283,
            "range": "± 12375",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11204,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "c76923f31bdc3f058035e8b337cb23f44e76192b",
          "message": "⬆️ micro: Update test-log to v0.2.18 (#1418)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [test-log](https://redirect.github.com/d-e-s-o/test-log) |\nworkspace.dependencies | patch | `0.2.17` -> `0.2.18` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>d-e-s-o/test-log (test-log)</summary>\n\n###\n[`v0.2.18`](https://redirect.github.com/d-e-s-o/test-log/blob/HEAD/CHANGELOG.md#0218)\n\n[Compare\nSource](https://redirect.github.com/d-e-s-o/test-log/compare/v0.2.17...v0.2.18)\n\n- Improved cooperation with other similar procedural macros to enable\n  attribute stacking\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MC42Mi4xIiwidXBkYXRlZEluVmVyIjoiNDAuNjIuMSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-06-30T16:37:58Z",
          "tree_id": "05480bb41a2b2d147a9219c6b3e54fdc527f8c50",
          "url": "https://github.com/dbus2/zbus/commit/c76923f31bdc3f058035e8b337cb23f44e76192b"
        },
        "date": 1751302157049,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2256,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3680846,
            "range": "± 32382",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4240673,
            "range": "± 5800",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 375,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 492,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 157,
            "range": "± 5",
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
            "value": 137,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 137,
            "range": "± 1",
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
            "value": 395223,
            "range": "± 10182",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 468418,
            "range": "± 1680",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1183594,
            "range": "± 11547",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2134081,
            "range": "± 5902",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2364639,
            "range": "± 11483",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3960125,
            "range": "± 14745",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1000561,
            "range": "± 11291",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1374134,
            "range": "± 2450",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11037,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 139,
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
          "id": "f9870cde4accffe48eb843a7dcfc3f53dc8a06b2",
          "message": "Merge pull request #1419 from zeenix/fix-zv-regression\n\n🚑️ zv: Check signature before serializing struct as a u8",
          "timestamp": "2025-07-01T00:03:07+02:00",
          "tree_id": "af5856967bcc3d5bd97ecada151ecb03ba9bacc5",
          "url": "https://github.com/dbus2/zbus/commit/f9870cde4accffe48eb843a7dcfc3f53dc8a06b2"
        },
        "date": 1751321678738,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2273,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3569878,
            "range": "± 18865",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 235,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4475009,
            "range": "± 8595",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 383,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 510,
            "range": "± 20",
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
            "value": 156,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 168,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 136,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 136,
            "range": "± 1",
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
            "value": 502384,
            "range": "± 16810",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 453053,
            "range": "± 1395",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1159523,
            "range": "± 5674",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2139114,
            "range": "± 5406",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2255156,
            "range": "± 7719",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4459053,
            "range": "± 18080",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1001120,
            "range": "± 1394",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1315690,
            "range": "± 7145",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10980,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "7ab8fa67ee5bffbbd40e12378a563893ebe2b68e",
          "message": "Merge pull request #1420 from dbus2/renovate/tokio-1.x-lockfile\n\n⬆️ Update tokio to v1.46.0",
          "timestamp": "2025-07-02T15:07:57+02:00",
          "tree_id": "ab2acf6efd87e81f588a878ef6eed4e91cbf64f2",
          "url": "https://github.com/dbus2/zbus/commit/7ab8fa67ee5bffbbd40e12378a563893ebe2b68e"
        },
        "date": 1751462361750,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2271,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3579416,
            "range": "± 18938",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3917899,
            "range": "± 4370",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 395,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 488,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 152,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 153,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 136,
            "range": "± 1",
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
            "value": 75,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 408081,
            "range": "± 6505",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 469206,
            "range": "± 2388",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1091839,
            "range": "± 4348",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2138632,
            "range": "± 8805",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2238215,
            "range": "± 7755",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4140129,
            "range": "± 19554",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1063981,
            "range": "± 3836",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1344848,
            "range": "± 3088",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11037,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "8250c5357eef47c7a2695ecdf8ac29a5b7ca7042",
          "message": "⬆️ micro: Update libfuzzer-sys to v0.4.10 (#1421)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [libfuzzer-sys](https://redirect.github.com/rust-fuzz/libfuzzer) |\ndependencies | patch | `0.4.9` -> `0.4.10` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>rust-fuzz/libfuzzer (libfuzzer-sys)</summary>\n\n###\n[`v0.4.10`](https://redirect.github.com/rust-fuzz/libfuzzer/blob/HEAD/CHANGELOG.md#0410)\n\n[Compare\nSource](https://redirect.github.com/rust-fuzz/libfuzzer/compare/0.4.9...0.4.10)\n\nReleased 2025-07-03.\n\n##### Changed\n\n- Updated to `libFuzzer` commit `6146a88f6049` (`release/20.x`).\n- Fuzz targets taking raw byte slice inputs can now return `Corpus`\nresults.\n\n***\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS4xNy4yIiwidXBkYXRlZEluVmVyIjoiNDEuMTcuMiIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-03T18:57:15Z",
          "tree_id": "68973cdb51fa3371559a3dccda328c9ee6ae6ae4",
          "url": "https://github.com/dbus2/zbus/commit/8250c5357eef47c7a2695ecdf8ac29a5b7ca7042"
        },
        "date": 1751569716432,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2238,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3729777,
            "range": "± 105875",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 232,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4142613,
            "range": "± 74175",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 398,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 495,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 151,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 155,
            "range": "± 2",
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
            "value": 133,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 131,
            "range": "± 0",
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
            "value": 414634,
            "range": "± 20713",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 445534,
            "range": "± 3188",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1208043,
            "range": "± 20340",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2114552,
            "range": "± 31500",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2358998,
            "range": "± 14520",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3934300,
            "range": "± 41090",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1001632,
            "range": "± 2394",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1255234,
            "range": "± 15983",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10876,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
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
          "id": "d01e893a8b3763cf3533bcf8dccf0ae79a331045",
          "message": "⬆️ micro: Update tokio to v1.46.1 (#1422)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [tokio](https://tokio.rs)\n([source](https://redirect.github.com/tokio-rs/tokio)) |\nworkspace.dependencies | patch | `1.46.0` -> `1.46.1` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>tokio-rs/tokio (tokio)</summary>\n\n###\n[`v1.46.1`](https://redirect.github.com/tokio-rs/tokio/releases/tag/tokio-1.46.1):\nTokio v1.46.1\n\n### 1.46.1 (July 4th, 2025)\n\nThis release fixes incorrect spawn locations in runtime task hooks for\ntasks\nspawned using `tokio::spawn` rather than `Runtime::spawn`. This issue\nonly\neffected the spawn location in `TaskMeta::spawned_at`, and did not\neffect task\nlocations in Tracing events.\n\n#### Unstable\n\n- runtime: add `TaskMeta::spawn_location` tracking where a task was\nspawned\n  ([#&#8203;7440])\n\n[#&#8203;7440]: https://redirect.github.com/tokio-rs/tokio/pull/7440\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS4xNy4yIiwidXBkYXRlZEluVmVyIjoiNDEuMTcuMiIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-04T20:13:02Z",
          "tree_id": "230e0143ef33bc11f10e13c1cbed6314b3d490f0",
          "url": "https://github.com/dbus2/zbus/commit/d01e893a8b3763cf3533bcf8dccf0ae79a331045"
        },
        "date": 1751660672763,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2205,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3632274,
            "range": "± 35984",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 231,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4198627,
            "range": "± 9069",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 408,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 507,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 158,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 156,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 168,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 135,
            "range": "± 3",
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
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 494793,
            "range": "± 24019",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 447152,
            "range": "± 691",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1171412,
            "range": "± 14427",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2120676,
            "range": "± 3723",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2302408,
            "range": "± 12142",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4289922,
            "range": "± 15932",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 1032230,
            "range": "± 5914",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1281605,
            "range": "± 2615",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11174,
            "range": "± 202",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 132,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}