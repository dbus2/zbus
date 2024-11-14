window.BENCHMARK_DATA = {
  "lastUpdate": 1731615932638,
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
          "id": "99f1664508258642bb1a41a869a3c50d89e6aaa9",
          "message": "Merge pull request #1130 from zeenix/better-git-hooks-suggestion\n\n👷 CONTRIBUTING: Suggest to copy the git hooks",
          "timestamp": "2024-11-07T22:59:12+01:00",
          "tree_id": "6c11c1738459d369a6ec94af4fe08e8236fd23ad",
          "url": "https://github.com/dbus2/zbus/commit/99f1664508258642bb1a41a869a3c50d89e6aaa9"
        },
        "date": 1731017439675,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2181,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3002522,
            "range": "± 20260",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 208,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3956912,
            "range": "± 13991",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 403,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 512,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 115,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 219750,
            "range": "± 545",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 424240,
            "range": "± 2881",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634519,
            "range": "± 1008",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2135395,
            "range": "± 11039",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1715148,
            "range": "± 8005",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4159814,
            "range": "± 11536",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166029,
            "range": "± 1149",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131497,
            "range": "± 2846",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10859,
            "range": "± 760",
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
            "email": "zeenix@gmail.com",
            "name": "Zeeshan Ali Khan",
            "username": "zeenix"
          },
          "distinct": true,
          "id": "fdca271cbe419f6e7938b505350d5ba745b060c0",
          "message": "🔖 zb,zm: Release 5.1.1",
          "timestamp": "2024-11-07T22:59:51+01:00",
          "tree_id": "cd9a747dac92a41b1c80089fe25addee2e50fdde",
          "url": "https://github.com/dbus2/zbus/commit/fdca271cbe419f6e7938b505350d5ba745b060c0"
        },
        "date": 1731017688910,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2159,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2960634,
            "range": "± 31125",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 221,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3961145,
            "range": "± 29891",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 517,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220088,
            "range": "± 1597",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 425443,
            "range": "± 1159",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630676,
            "range": "± 1631",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2145235,
            "range": "± 6207",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1732059,
            "range": "± 13175",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3934041,
            "range": "± 52018",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166125,
            "range": "± 427",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1173955,
            "range": "± 6758",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10802,
            "range": "± 861",
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
          "id": "525706fd9c3a09178b4d8f44a82acdd22e49d9bd",
          "message": "⬆️ micro: Update libfuzzer-sys to v0.4.8 (#1131)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [libfuzzer-sys](https://redirect.github.com/rust-fuzz/libfuzzer) |\ndependencies | patch | `0.4.7` -> `0.4.8` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>rust-fuzz/libfuzzer (libfuzzer-sys)</summary>\n\n###\n[`v0.4.8`](https://redirect.github.com/rust-fuzz/libfuzzer/blob/HEAD/CHANGELOG.md#048)\n\n[Compare\nSource](https://redirect.github.com/rust-fuzz/libfuzzer/compare/0.4.7...0.4.8)\n\nReleased 2024-11-07.\n\n##### Added\n\n-   Bindings to `LLVMFuzzerCustomCrossOver` through the `fuzz_crossover`\nmacro. See the `example_crossover` directory in this crate's repo for a\n    complete example.\n\n##### Changed\n\n- Updated to `libFuzzer` commit\n`ab51eccf88f5321e7c60591c5546b254b6afab99`\n    (`release/19.x`).\n\n***\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS43LjEiLCJ1cGRhdGVkSW5WZXIiOiIzOS43LjEiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-07T22:11:14Z",
          "tree_id": "aa4c7464ea4c73b389216321c9cd18bccb1e2691",
          "url": "https://github.com/dbus2/zbus/commit/525706fd9c3a09178b4d8f44a82acdd22e49d9bd"
        },
        "date": 1731018145336,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2162,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3030158,
            "range": "± 31221",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4065644,
            "range": "± 86650",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 510,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220911,
            "range": "± 2936",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423985,
            "range": "± 2277",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 635852,
            "range": "± 3341",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2127840,
            "range": "± 24009",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1726639,
            "range": "± 30181",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3888242,
            "range": "± 14926",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166466,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1130159,
            "range": "± 8538",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10866,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 137,
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
          "id": "7e5345157824efdead6e8de86336b8d48ce223ba",
          "message": "Merge pull request #1132 from dbus2/renovate/async-io-2.x-lockfile\n\n⬆️ Update async-io to v2.4.0",
          "timestamp": "2024-11-07T23:13:32+01:00",
          "tree_id": "05eed8d45e6cb8738f02bf18892dc5cb3826f6a9",
          "url": "https://github.com/dbus2/zbus/commit/7e5345157824efdead6e8de86336b8d48ce223ba"
        },
        "date": 1731018309751,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2203,
            "range": "± 59",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2962656,
            "range": "± 20047",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 226,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4379765,
            "range": "± 23105",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 531,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 104,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 111,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 106,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 221151,
            "range": "± 8090",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 425353,
            "range": "± 2465",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631496,
            "range": "± 2992",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2141797,
            "range": "± 4952",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1766622,
            "range": "± 2879",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4114340,
            "range": "± 22026",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 165919,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1133034,
            "range": "± 15541",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10846,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 136,
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
          "id": "9be25a55937318ffcc892d98801481aebd7fdf9c",
          "message": "Merge pull request #1133 from dbus2/renovate/tempfile-3.x-lockfile\n\n⬆️ Update tempfile to v3.14.0",
          "timestamp": "2024-11-08T11:06:25+01:00",
          "tree_id": "367e25f896697858d65d6fa5f54ec95e0ea35823",
          "url": "https://github.com/dbus2/zbus/commit/9be25a55937318ffcc892d98801481aebd7fdf9c"
        },
        "date": 1731061076433,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2210,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3739191,
            "range": "± 50812",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 219,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4062396,
            "range": "± 58625",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 407,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 519,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 103,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 101,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 75,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 216543,
            "range": "± 1927",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 415493,
            "range": "± 2728",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 630322,
            "range": "± 4245",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2102981,
            "range": "± 8147",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1816321,
            "range": "± 17092",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3940109,
            "range": "± 40868",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166237,
            "range": "± 794",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1134472,
            "range": "± 3698",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11175,
            "range": "± 58",
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
          "id": "415404479c20754a2fe0acde6e4eaf89f0e43cca",
          "message": "⬆️ micro: Update JamesIves/github-pages-deploy-action action to v4.6.9 (#1134)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n|\n[JamesIves/github-pages-deploy-action](https://redirect.github.com/JamesIves/github-pages-deploy-action)\n| action | patch | `v4.6.8` -> `v4.6.9` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>JamesIves/github-pages-deploy-action\n(JamesIves/github-pages-deploy-action)</summary>\n\n###\n[`v4.6.9`](https://redirect.github.com/JamesIves/github-pages-deploy-action/releases/tag/v4.6.9)\n\n[Compare\nSource](https://redirect.github.com/JamesIves/github-pages-deploy-action/compare/v4.6.8...v4.6.9)\n\n<!-- Release notes generated using configuration in .github/release.yml\nat releases/v4 -->\n\n#### What's Changed\n\n##### Dependencies 🤖\n\n-   chore(deps): mass bump dependencies\n- chore(deps): switch to using `.node-version` instead of `.nvmrc` for\nNode dependency management.\n-   chore(deps): updated node version to 22.11.0 for development\n\n**Full Changelog**:\nhttps://github.com/JamesIves/github-pages-deploy-action/compare/v4...v4.6.9\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS43LjEiLCJ1cGRhdGVkSW5WZXIiOiIzOS43LjEiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-09T21:51:32Z",
          "tree_id": "0190a07b7965605f9e93c66384bc66274e237c75",
          "url": "https://github.com/dbus2/zbus/commit/415404479c20754a2fe0acde6e4eaf89f0e43cca"
        },
        "date": 1731189774322,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2106,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2950937,
            "range": "± 20635",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 217,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3910725,
            "range": "± 16033",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 421,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 523,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 116,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 101,
            "range": "± 3",
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
            "value": 104,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 222814,
            "range": "± 1496",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423965,
            "range": "± 706",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 638886,
            "range": "± 927",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2111396,
            "range": "± 6343",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1756826,
            "range": "± 8077",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3954605,
            "range": "± 16153",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166003,
            "range": "± 189",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1132812,
            "range": "± 2410",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10857,
            "range": "± 28",
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
          "id": "ae3404ea28a97c051e477b6b498461fe12d0bd8b",
          "message": "Merge pull request #1136 from zeenix/introspect-bloat\n\n⚡️ zm: interface allows to disable docs in introspection + do that for fdo interfaces",
          "timestamp": "2024-11-12T00:46:59+01:00",
          "tree_id": "087ccf3e1444b2923019f90982ca9e4c62ccd886",
          "url": "https://github.com/dbus2/zbus/commit/ae3404ea28a97c051e477b6b498461fe12d0bd8b"
        },
        "date": 1731369492911,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2135,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2944026,
            "range": "± 49743",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3981808,
            "range": "± 16474",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 515,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 108,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 116,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 99,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 221902,
            "range": "± 3846",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 422116,
            "range": "± 1957",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 635925,
            "range": "± 1424",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2127248,
            "range": "± 18126",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1756704,
            "range": "± 11661",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3798466,
            "range": "± 9557",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166517,
            "range": "± 432",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1132763,
            "range": "± 11160",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10979,
            "range": "± 26",
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
          "id": "9ddd3d0f2c0700e67fb23f2c6300055f02c9b032",
          "message": "⬆️ micro: Update serde to v1.0.215 (#1137)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) | dependencies |\npatch | `1.0.214` -> `1.0.215` |\n| [serde](https://serde.rs)\n([source](https://redirect.github.com/serde-rs/serde)) |\ndev-dependencies | patch | `1.0.214` -> `1.0.215` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/serde (serde)</summary>\n\n###\n[`v1.0.215`](https://redirect.github.com/serde-rs/serde/releases/tag/v1.0.215)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/serde/compare/v1.0.214...v1.0.215)\n\n- Produce warning when multiple fields or variants have the same\ndeserialization name\n([#&#8203;2855](https://redirect.github.com/serde-rs/serde/issues/2855),\n[#&#8203;2856](https://redirect.github.com/serde-rs/serde/issues/2856),\n[#&#8203;2857](https://redirect.github.com/serde-rs/serde/issues/2857))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about these\nupdates again.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45LjUiLCJ1cGRhdGVkSW5WZXIiOiIzOS45LjUiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-11T23:58:55Z",
          "tree_id": "a20f677452baaf48785ac03c1d082775adebfe7f",
          "url": "https://github.com/dbus2/zbus/commit/9ddd3d0f2c0700e67fb23f2c6300055f02c9b032"
        },
        "date": 1731370219187,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2188,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2907590,
            "range": "± 25679",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 238,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3848056,
            "range": "± 30717",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 418,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 517,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 109,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 100,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 222662,
            "range": "± 6254",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 423666,
            "range": "± 628",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 629941,
            "range": "± 4553",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2116306,
            "range": "± 17631",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1758579,
            "range": "± 5329",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3870709,
            "range": "± 29420",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166205,
            "range": "± 459",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131079,
            "range": "± 7462",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10892,
            "range": "± 26",
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
          "id": "1b5c4f0e6fc61b7d103849a160629d4bccc9cb7b",
          "message": "Merge pull request #1138 from zeenix/no-proxy-in-conn\n\n⚡️ zb: Don't use proxies in connection code",
          "timestamp": "2024-11-12T12:12:34+01:00",
          "tree_id": "eb90b5a2fcfb7c15e5033a4e278df2ad2fd83887",
          "url": "https://github.com/dbus2/zbus/commit/1b5c4f0e6fc61b7d103849a160629d4bccc9cb7b"
        },
        "date": 1731410639382,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2164,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2973144,
            "range": "± 30589",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 228,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3923416,
            "range": "± 37805",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 420,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 523,
            "range": "± 38",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 117,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 116,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 213240,
            "range": "± 1169",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 417958,
            "range": "± 948",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 618391,
            "range": "± 1567",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2123387,
            "range": "± 14452",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1769283,
            "range": "± 10893",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3855727,
            "range": "± 14348",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166399,
            "range": "± 226",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1118335,
            "range": "± 4811",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10914,
            "range": "± 33",
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
          "id": "ec45ef4bd43c90adfa9e20af1354b043ac81a553",
          "message": "⬆️ micro: Update glib to v0.20.6 (#1139)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [glib](https://gtk-rs.org/)\n([source](https://redirect.github.com/gtk-rs/gtk-rs-core)) |\ndev-dependencies | patch | `0.20.5` -> `0.20.6` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>gtk-rs/gtk-rs-core (glib)</summary>\n\n###\n[`v0.20.6`](https://redirect.github.com/gtk-rs/gtk-rs-core/releases/tag/0.20.6)\n\n[Compare\nSource](https://redirect.github.com/gtk-rs/gtk-rs-core/compare/0.20.5...0.20.6)\n\n    François Laignel:\n          glib: fix userdata mutability for FnMut callbacks\n\n    Sebastian Dröge:\n          Update gir\n          Regenerate with latest gir\n          pango: Fix `LayoutLine::x_ranges()` bindings\n          Update Cargo.lock\n          Update gir-files\n          Regenerate with latest gir / gir-files\n          gio: Clean up and autogenerate `UnixMountEntry` bindings\n          glib: Ignore CPP feature constants\n          glib: Add correct versions to various new unicode scripts\n          Update versions to 0.20.6\n          Update glib-sys / gio-sys dependency version\n\n    Sebastian Wiesner:\n          Fix typo to get Github actions to pass\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45LjUiLCJ1cGRhdGVkSW5WZXIiOiIzOS45LjUiLCJ0YXJnZXRCcmFuY2giOiJtYWluIiwibGFiZWxzIjpbXX0=-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-12T13:49:00Z",
          "tree_id": "a47f8d46c4b825737fb05c9da48fbfd82b2dec58",
          "url": "https://github.com/dbus2/zbus/commit/ec45ef4bd43c90adfa9e20af1354b043ac81a553"
        },
        "date": 1731420012740,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2213,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3070584,
            "range": "± 28945",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 226,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3873020,
            "range": "± 38190",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 410,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 530,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 115,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 117,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 103,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 214460,
            "range": "± 548",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 419471,
            "range": "± 1481",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624764,
            "range": "± 1266",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2121366,
            "range": "± 18561",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1742397,
            "range": "± 10422",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3866133,
            "range": "± 6326",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166701,
            "range": "± 10353",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1122662,
            "range": "± 2352",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10866,
            "range": "± 37",
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
          "id": "e542606e7e05f3b65128b2bdbe369d1b41a33edd",
          "message": "⬆️ micro: Update clap to v4.5.21 (#1140)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [clap](https://redirect.github.com/clap-rs/clap) | dependencies |\npatch | `4.5.20` -> `4.5.21` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>clap-rs/clap (clap)</summary>\n\n###\n[`v4.5.21`](https://redirect.github.com/clap-rs/clap/blob/HEAD/CHANGELOG.md#4521---2024-11-13)\n\n##### Fixes\n\n- *(parser)* Ensure defaults are filled in on error with\n`ignore_errors(true)`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xMS41IiwidXBkYXRlZEluVmVyIjoiMzkuMTEuNSIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2024-11-13T20:09:04Z",
          "tree_id": "6cc9a1b433568540c4b08a6710c6ec1dce419b5a",
          "url": "https://github.com/dbus2/zbus/commit/e542606e7e05f3b65128b2bdbe369d1b41a33edd"
        },
        "date": 1731529212374,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2178,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2954376,
            "range": "± 74183",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 234,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3992414,
            "range": "± 10727",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 457,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 560,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 211366,
            "range": "± 3648",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 407906,
            "range": "± 1397",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 619401,
            "range": "± 2731",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2105579,
            "range": "± 8261",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1721669,
            "range": "± 7362",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3781115,
            "range": "± 15759",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166481,
            "range": "± 702",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1122022,
            "range": "± 1895",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10783,
            "range": "± 35",
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
          "id": "2a5e69cf3f108db66493a2c2a0527c5eed8e73dd",
          "message": "Merge pull request #1141 from dbus2/renovate/tokio-vsock-0.x\n\n⬆️ Update tokio-vsock to 0.6",
          "timestamp": "2024-11-13T21:08:49+01:00",
          "tree_id": "d16b0cb2bb77dad2ee4e3bdd1b19ee403f78a0c4",
          "url": "https://github.com/dbus2/zbus/commit/2a5e69cf3f108db66493a2c2a0527c5eed8e73dd"
        },
        "date": 1731529228560,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2112,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2950965,
            "range": "± 35989",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 250,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4030741,
            "range": "± 45362",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 414,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 513,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 114,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 212320,
            "range": "± 2080",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 418802,
            "range": "± 3737",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631253,
            "range": "± 2852",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2115608,
            "range": "± 28562",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1711801,
            "range": "± 12859",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3967771,
            "range": "± 29671",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168934,
            "range": "± 2758",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1131548,
            "range": "± 4869",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10960,
            "range": "± 30",
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
          "id": "93d477c8f0f783626898d796b67390dcca727553",
          "message": "Merge pull request #1142 from zeenix/drop-sink-feature\n\n🔥 zb: Drop now unused `sink` feature of `futures-util`",
          "timestamp": "2024-11-13T22:44:06+01:00",
          "tree_id": "7ee250f9f840d3eb29039750ed96b1338fc09108",
          "url": "https://github.com/dbus2/zbus/commit/93d477c8f0f783626898d796b67390dcca727553"
        },
        "date": 1731534923260,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2164,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2926431,
            "range": "± 29968",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 244,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3881591,
            "range": "± 21558",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 419,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 524,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 115,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 105,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 222789,
            "range": "± 2770",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 426770,
            "range": "± 6931",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 626763,
            "range": "± 3227",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2130536,
            "range": "± 42022",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1737729,
            "range": "± 47494",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3881939,
            "range": "± 18019",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166412,
            "range": "± 1014",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1130834,
            "range": "± 17245",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11069,
            "range": "± 51",
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
          "id": "737b3efa2e454dfe556befbb1b97ad4cfe03dcc3",
          "message": "Merge pull request #1143 from zeenix/gvariant-patch\n\n🚩 zm: Add `gvariant` feature flag",
          "timestamp": "2024-11-13T22:50:54+01:00",
          "tree_id": "524c1613c3ff99d773d9103b888623ac105f1fe3",
          "url": "https://github.com/dbus2/zbus/commit/737b3efa2e454dfe556befbb1b97ad4cfe03dcc3"
        },
        "date": 1731535349049,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2145,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3002536,
            "range": "± 36033",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 258,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3868499,
            "range": "± 27925",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 413,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 514,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 109,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 106,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 115,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 105,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 104,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220615,
            "range": "± 2880",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 427151,
            "range": "± 1703",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 634006,
            "range": "± 7975",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2129036,
            "range": "± 13882",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1742263,
            "range": "± 13555",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4056444,
            "range": "± 10699",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 166707,
            "range": "± 532",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1170766,
            "range": "± 4596",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11041,
            "range": "± 24",
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
          "id": "055bde5867fe8556d4cebc541c34afc9773db170",
          "message": "Merge pull request #1146 from zeenix/skip-self\n\n🔊 zb: Skip `self` in an instrumented method",
          "timestamp": "2024-11-14T21:14:17+01:00",
          "tree_id": "5098e9bda3caeb74f2701fe471919e5ff13fcdab",
          "url": "https://github.com/dbus2/zbus/commit/055bde5867fe8556d4cebc541c34afc9773db170"
        },
        "date": 1731615931662,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2179,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2932929,
            "range": "± 36419",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 216,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3859464,
            "range": "± 28187",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 419,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 526,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 111,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 115,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 102,
            "range": "± 3",
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
            "value": 93,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218540,
            "range": "± 3847",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 410029,
            "range": "± 1919",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 636762,
            "range": "± 2553",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2139816,
            "range": "± 18742",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1742242,
            "range": "± 6062",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3995918,
            "range": "± 38216",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168689,
            "range": "± 556",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1136258,
            "range": "± 15082",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10856,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 99,
            "range": "± 3",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}