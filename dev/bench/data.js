window.BENCHMARK_DATA = {
  "lastUpdate": 1752118124760,
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
          "id": "b7c16006f3b79090c86f04b7934611e7796a6af1",
          "message": "⬆️ micro: Update clap to v4.5.41 (#1427)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) |\nworkspace.dependencies | patch | `4.5.40` -> `4.5.41` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.41`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4541---2025-07-09)\n\n[Compare\nSource](https://redirect.github.com/clap-rs/clap/compare/v4.5.40...v4.5.41)\n\n##### Features\n\n- Add `Styles::context` and `Styles::context_value` to customize the\nstyling of `[default: value]` like notes in the `--help`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiI0MS4yMy4yIiwidXBkYXRlZEluVmVyIjoiNDEuMjMuMiIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-07-10T03:17:01Z",
          "tree_id": "226c68d0c1a77fd74d1fe0039e0d6a516e9b5dc1",
          "url": "https://github.com/dbus2/zbus/commit/b7c16006f3b79090c86f04b7934611e7796a6af1"
        },
        "date": 1752118123038,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2279,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3536487,
            "range": "± 34045",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 233,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4437031,
            "range": "± 28493",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 393,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 491,
            "range": "± 10",
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
            "value": 158,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 167,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 135,
            "range": "± 6",
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
            "value": 101,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 469727,
            "range": "± 22474",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 456158,
            "range": "± 2423",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 1138776,
            "range": "± 29249",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2446832,
            "range": "± 49247",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 2278385,
            "range": "± 14376",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4482704,
            "range": "± 16556",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 973015,
            "range": "± 5600",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1317007,
            "range": "± 11065",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10989,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 138,
            "range": "± 0",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}