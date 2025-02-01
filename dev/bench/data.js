window.BENCHMARK_DATA = {
  "lastUpdate": 1738448481235,
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
          "id": "c3129816d6318de3673a9433a1393cc26a48b579",
          "message": "🔖 zb,zm: Release 5.3.1",
          "timestamp": "2025-01-23T16:47:48+01:00",
          "tree_id": "108147fae8fb926213c48ba699cecdd81061affd",
          "url": "https://github.com/dbus2/zbus/commit/c3129816d6318de3673a9433a1393cc26a48b579"
        },
        "date": 1737647973801,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2081,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2899007,
            "range": "± 37167",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 215,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4131950,
            "range": "± 14414",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 415,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 505,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 105,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 112,
            "range": "± 5",
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
            "range": "± 4",
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
            "value": 221182,
            "range": "± 1266",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412114,
            "range": "± 1091",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 646894,
            "range": "± 4011",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2077306,
            "range": "± 16675",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1704550,
            "range": "± 6479",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3956368,
            "range": "± 26544",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167089,
            "range": "± 981",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1126300,
            "range": "± 1526",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11060,
            "range": "± 378",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 112,
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
          "id": "3be463419c082ff9ad46993e58aff0ea89e040c8",
          "message": "Merge pull request #1225 from dbus2/renovate/rand-0.x\n\n⬆️ Update rand to 0.9.0",
          "timestamp": "2025-01-27T21:50:18+01:00",
          "tree_id": "ab48cee9c75f485af4ecd1fc2b127bc71d6a824d",
          "url": "https://github.com/dbus2/zbus/commit/3be463419c082ff9ad46993e58aff0ea89e040c8"
        },
        "date": 1738011679644,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2232,
            "range": "± 52",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3110502,
            "range": "± 37134",
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
            "value": 3954749,
            "range": "± 18703",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 402,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 501,
            "range": "± 11",
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
            "value": 114,
            "range": "± 2",
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
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218826,
            "range": "± 3210",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 411747,
            "range": "± 1931",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 636603,
            "range": "± 3537",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2088054,
            "range": "± 4369",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1704070,
            "range": "± 16777",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3880751,
            "range": "± 10346",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168181,
            "range": "± 588",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1142888,
            "range": "± 4188",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11009,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
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
          "id": "eab109b5c35f6f91b077a511103b903a0135d900",
          "message": "⬆️ micro: Update winnow to v0.6.25 (#1226)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [winnow](https://redirect.github.com/winnow-rs/winnow) | dependencies\n| patch | `0.6.24` -> `0.6.25` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>winnow-rs/winnow (winnow)</summary>\n\n###\n[`v0.6.25`](https://redirect.github.com/winnow-rs/winnow/blob/HEAD/CHANGELOG.md#0625---2025-01-27)\n\n[Compare\nSource](https://redirect.github.com/winnow-rs/winnow/compare/v0.6.24...v0.6.25)\n\n##### Compatibility\n\n- Deprecated `PResult` in favor of `ModalResult`: v0.7 will make\n`ErrMode` optional and `PResult` will no longer be descriptive enough\n-   Deprecate `IResult` in favor of `PResult<(I, O)>`\n\n##### Documentation\n\n-   Update comparison with nom\n\n##### Fixes\n\n-   Ensure we append errors in `repeat(_).fold(1..)`\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xMjUuMSIsInVwZGF0ZWRJblZlciI6IjM5LjEyNS4xIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-28T01:40:36Z",
          "tree_id": "15c430b16bf981382d290838774862be8084592b",
          "url": "https://github.com/dbus2/zbus/commit/eab109b5c35f6f91b077a511103b903a0135d900"
        },
        "date": 1738029092819,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2136,
            "range": "± 43",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3072842,
            "range": "± 27900",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 264,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3884594,
            "range": "± 28271",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 512,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 115,
            "range": "± 2",
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
            "value": 104,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 104,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220171,
            "range": "± 1250",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412862,
            "range": "± 2237",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 641718,
            "range": "± 57498",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2091292,
            "range": "± 5781",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1737562,
            "range": "± 6185",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3813111,
            "range": "± 33655",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 169629,
            "range": "± 418",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1147679,
            "range": "± 2330",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11064,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
            "range": "± 2",
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
          "id": "d9809bd9683d7fae999b0800d35ba8212cc5c2f6",
          "message": "⬆️ micro: Update libfuzzer-sys to v0.4.9 (#1227)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [libfuzzer-sys](https://redirect.github.com/rust-fuzz/libfuzzer) |\ndependencies | patch | `0.4.8` -> `0.4.9` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>rust-fuzz/libfuzzer (libfuzzer-sys)</summary>\n\n###\n[`v0.4.9`](https://redirect.github.com/rust-fuzz/libfuzzer/blob/HEAD/CHANGELOG.md#049)\n\n[Compare\nSource](https://redirect.github.com/rust-fuzz/libfuzzer/compare/0.4.8...0.4.9)\n\nReleased 2025-01-28.\n\n##### Added\n\n- The `example_init` demonstrates how to pass an initialization code\nblock to\n    the `fuzz_target!` macro.\n\n##### Changed\n\n- The `fuzz_target!` macro now supports the generation of\n`LLVMFuzzerInitialize`\nto execute initialization code once before running the fuzzer. This\nchange is\n    not breaking and is completely backward compatible.\n\n***\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xMjUuMSIsInVwZGF0ZWRJblZlciI6IjM5LjEyNS4xIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-28T22:07:43Z",
          "tree_id": "5fae83524741b58ca4ec6c4061fa5a2d8d923ea0",
          "url": "https://github.com/dbus2/zbus/commit/d9809bd9683d7fae999b0800d35ba8212cc5c2f6"
        },
        "date": 1738102741231,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2120,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2915733,
            "range": "± 22718",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3829068,
            "range": "± 13277",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 409,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 514,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 114,
            "range": "± 2",
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
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 103,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 93,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 220341,
            "range": "± 1049",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 415248,
            "range": "± 2194",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 636476,
            "range": "± 1837",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2105342,
            "range": "± 16498",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1747200,
            "range": "± 10703",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4041172,
            "range": "± 18125",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167705,
            "range": "± 392",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1145094,
            "range": "± 2867",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11256,
            "range": "± 79",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
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
          "id": "c2ee8968551f9eab062161a280504d59adff7107",
          "message": "⬆️ micro: Update serde_json to v1.0.138 (#1228)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [serde_json](https://redirect.github.com/serde-rs/json) |\ndev-dependencies | patch | `1.0.137` -> `1.0.138` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>serde-rs/json (serde_json)</summary>\n\n###\n[`v1.0.138`](https://redirect.github.com/serde-rs/json/releases/tag/v1.0.138)\n\n[Compare\nSource](https://redirect.github.com/serde-rs/json/compare/v1.0.137...v1.0.138)\n\n-   Documentation improvements\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xMjUuMSIsInVwZGF0ZWRJblZlciI6IjM5LjEyNS4xIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-28T22:08:45Z",
          "tree_id": "001bb65f9eacc4e5bd39ede0fdd6f746ea00b845",
          "url": "https://github.com/dbus2/zbus/commit/c2ee8968551f9eab062161a280504d59adff7107"
        },
        "date": 1738102798121,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2241,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2930569,
            "range": "± 23603",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 243,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3826667,
            "range": "± 10709",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 420,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 508,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 106,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 107,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 112,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 106,
            "range": "± 2",
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
            "value": 105,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 218646,
            "range": "± 2165",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 414852,
            "range": "± 971",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 637516,
            "range": "± 7713",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2046791,
            "range": "± 8541",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1712589,
            "range": "± 4915",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3817953,
            "range": "± 10005",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167072,
            "range": "± 453",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1178064,
            "range": "± 3457",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11060,
            "range": "± 45",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 113,
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
          "id": "c97ddc01c5ff1ee7f225d1660c02cf7a0f21da63",
          "message": "Merge pull request #1229 from dbus2/renovate/tempfile-3.x-lockfile\n\n⬆️ Update tempfile to v3.16.0",
          "timestamp": "2025-01-29T02:46:32+01:00",
          "tree_id": "4d58f20ba5b1657c71c50722c122515a467df1c5",
          "url": "https://github.com/dbus2/zbus/commit/c97ddc01c5ff1ee7f225d1660c02cf7a0f21da63"
        },
        "date": 1738115861299,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2169,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 2926679,
            "range": "± 10853",
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
            "value": 3827284,
            "range": "± 14429",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 405,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 504,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 100,
            "range": "± 4",
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
            "value": 107,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 110,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 107,
            "range": "± 3",
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
            "value": 218431,
            "range": "± 2326",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 415983,
            "range": "± 2833",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631235,
            "range": "± 2216",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2092403,
            "range": "± 6696",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1715765,
            "range": "± 6967",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3789654,
            "range": "± 47892",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 168346,
            "range": "± 822",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1129406,
            "range": "± 6839",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11149,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 113,
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
          "id": "725c40370423e873dfad75631510b5ec55af1634",
          "message": "Merge pull request #1230 from manuelheiss-liebherr/feature/consider-noreplyflag-in-interface-methods\n\n🩹 zm: do not reply if no-reply-expected flag is set",
          "timestamp": "2025-01-30T13:50:29+01:00",
          "tree_id": "a8e187a937d90203ca3a6e723d63d9a9a11a5ca6",
          "url": "https://github.com/dbus2/zbus/commit/725c40370423e873dfad75631510b5ec55af1634"
        },
        "date": 1738242108396,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2186,
            "range": "± 16",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3309312,
            "range": "± 26717",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3866135,
            "range": "± 12131",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 418,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 508,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 111,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 110,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 117,
            "range": "± 4",
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
            "value": 102,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 94,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 217656,
            "range": "± 1896",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412716,
            "range": "± 1375",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 638519,
            "range": "± 5633",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2066347,
            "range": "± 5849",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1668771,
            "range": "± 22748",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3764544,
            "range": "± 23403",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167704,
            "range": "± 940",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1139607,
            "range": "± 4112",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10916,
            "range": "± 1043",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 115,
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
          "id": "1f0208b474944577e5c4872a4d5ba85c50835348",
          "message": "Merge pull request #1231 from dbus2/renovate/winnow-0.x\n\n⬆️ Update winnow to 0.7",
          "timestamp": "2025-01-30T23:31:53+01:00",
          "tree_id": "06b16693bdaeb032d5305ce84af10b5597d4d38c",
          "url": "https://github.com/dbus2/zbus/commit/1f0208b474944577e5c4872a4d5ba85c50835348"
        },
        "date": 1738276990503,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2311,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3196919,
            "range": "± 33773",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 257,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4036803,
            "range": "± 29398",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 387,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 485,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 154,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 164,
            "range": "± 3",
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
            "range": "± 3",
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
            "value": 217008,
            "range": "± 1444",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 405899,
            "range": "± 2507",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 633944,
            "range": "± 5969",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2061452,
            "range": "± 12914",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1800166,
            "range": "± 8760",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4305806,
            "range": "± 15170",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167065,
            "range": "± 353",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1137030,
            "range": "± 2274",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11270,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 113,
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
          "id": "bfa282b781ed431097b9cd8f5e82b0d8c03ce014",
          "message": "⬆️ micro: Update syn to v2.0.96 (#1208)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) | dependencies | patch\n| `2.0.95` -> `2.0.96` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.96`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.96)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.95...2.0.96)\n\n- Expression precedence fixes\n([#&#8203;1834](https://redirect.github.com/dtolnay/syn/issues/1834),\n[#&#8203;1836](https://redirect.github.com/dtolnay/syn/issues/1836),\n[#&#8203;1837](https://redirect.github.com/dtolnay/syn/issues/1837),\n[#&#8203;1838](https://redirect.github.com/dtolnay/syn/issues/1838),\n[#&#8203;1839](https://redirect.github.com/dtolnay/syn/issues/1839),\n[#&#8203;1840](https://redirect.github.com/dtolnay/syn/issues/1840))\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS45Mi4wIiwidXBkYXRlZEluVmVyIjoiMzkuOTIuMCIsInRhcmdldEJyYW5jaCI6Im1haW4iLCJsYWJlbHMiOltdfQ==-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-01-30T22:43:12Z",
          "tree_id": "533386b0336d3ac55336c176999e4de7231be0cd",
          "url": "https://github.com/dbus2/zbus/commit/bfa282b781ed431097b9cd8f5e82b0d8c03ce014"
        },
        "date": 1738277667925,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2314,
            "range": "± 67",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3430777,
            "range": "± 23500",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 237,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4130218,
            "range": "± 20225",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 406,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 503,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 154,
            "range": "± 4",
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
            "value": 164,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/interface",
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/member",
            "value": 105,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_ser",
            "value": 216178,
            "range": "± 1373",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 414858,
            "range": "± 3030",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 631084,
            "range": "± 3181",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2057578,
            "range": "± 8497",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1776957,
            "range": "± 14921",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3954758,
            "range": "± 8374",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167256,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1180979,
            "range": "± 2577",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11181,
            "range": "± 283",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 113,
            "range": "± 2",
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
          "id": "9accfe305d3777d0452e3de474c78c645a3fcfe0",
          "message": "⬆️ micro: Update async-trait to v0.1.86 (#1232)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [async-trait](https://redirect.github.com/dtolnay/async-trait) |\ndependencies | patch | `0.1.85` -> `0.1.86` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/async-trait (async-trait)</summary>\n\n###\n[`v0.1.86`](https://redirect.github.com/dtolnay/async-trait/releases/tag/0.1.86)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/async-trait/compare/0.1.85...0.1.86)\n\n-   Documentation improvements\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xNDUuMCIsInVwZGF0ZWRJblZlciI6IjM5LjE0NS4wIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-02-01T05:07:11Z",
          "tree_id": "bd50bad873252a56d1a2533cc8dd908f239242ca",
          "url": "https://github.com/dbus2/zbus/commit/9accfe305d3777d0452e3de474c78c645a3fcfe0"
        },
        "date": 1738387109005,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2321,
            "range": "± 58",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3114628,
            "range": "± 32278",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 250,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 3969933,
            "range": "± 84423",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 408,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/tcp",
            "value": 499,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/well_known",
            "value": 155,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 154,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/bus",
            "value": 163,
            "range": "± 2",
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
            "value": 130,
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
            "value": 211293,
            "range": "± 3831",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412087,
            "range": "± 4236",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 624181,
            "range": "± 1845",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2079886,
            "range": "± 7343",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1841808,
            "range": "± 3989",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 3999560,
            "range": "± 51240",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167460,
            "range": "± 471",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1116191,
            "range": "± 5320",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 10953,
            "range": "± 121",
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
          "id": "04a01e0a8b85a58fd23924a718c1d05e8add6f7b",
          "message": "⬆️ micro: Update syn to v2.0.97 (#1235)\n\nThis PR contains the following updates:\n\n| Package | Type | Update | Change |\n|---|---|---|---|\n| [syn](https://redirect.github.com/dtolnay/syn) | dependencies | patch\n| `2.0.96` -> `2.0.97` |\n\n---\n\n### Release Notes\n\n<details>\n<summary>dtolnay/syn (syn)</summary>\n\n###\n[`v2.0.97`](https://redirect.github.com/dtolnay/syn/releases/tag/2.0.97)\n\n[Compare\nSource](https://redirect.github.com/dtolnay/syn/compare/2.0.96...2.0.97)\n\n-   Documentation improvements\n\n</details>\n\n---\n\n### Configuration\n\n📅 **Schedule**: Branch creation - At any time (no schedule defined),\nAutomerge - At any time (no schedule defined).\n\n🚦 **Automerge**: Enabled.\n\n♻ **Rebasing**: Whenever PR becomes conflicted, or you tick the\nrebase/retry checkbox.\n\n🔕 **Ignore**: Close this PR and you won't be reminded about this update\nagain.\n\n---\n\n- [ ] <!-- rebase-check -->If you want to rebase/retry this PR, check\nthis box\n\n---\n\nThis PR was generated by [Mend Renovate](https://mend.io/renovate/).\nView the [repository job\nlog](https://developer.mend.io/github/dbus2/zbus).\n\n<!--renovate-debug:eyJjcmVhdGVkSW5WZXIiOiIzOS4xNDUuMCIsInVwZGF0ZWRJblZlciI6IjM5LjE0NS4wIiwidGFyZ2V0QnJhbmNoIjoibWFpbiIsImxhYmVscyI6W119-->\n\nCo-authored-by: renovate[bot] <29139614+renovate[bot]@users.noreply.github.com>",
          "timestamp": "2025-02-01T22:10:04Z",
          "tree_id": "7c11f660ba427f5cbe9bcc6e9b96d65fb79c3312",
          "url": "https://github.com/dbus2/zbus/commit/04a01e0a8b85a58fd23924a718c1d05e8add6f7b"
        },
        "date": 1738448480080,
        "tool": "cargo",
        "benches": [
          {
            "name": "message-ser/small",
            "value": 2168,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "message-ser/big",
            "value": 3208195,
            "range": "± 27616",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/header",
            "value": 249,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "message-de/body",
            "value": 4046445,
            "range": "± 27371",
            "unit": "ns/iter"
          },
          {
            "name": "parse_dbus_address/unix",
            "value": 412,
            "range": "± 19",
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
            "value": 153,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/unique",
            "value": 154,
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
            "value": 127,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "parse_name/error",
            "value": 129,
            "range": "± 3",
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
            "value": 215059,
            "range": "± 1404",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_de",
            "value": 412851,
            "range": "± 2938",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_ser",
            "value": 632077,
            "range": "± 2312",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_ass_dict_de",
            "value": 2071177,
            "range": "± 7231",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_ser",
            "value": 1795145,
            "range": "± 17270",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/big_array_and_asv_dict_de",
            "value": 4114264,
            "range": "± 13265",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_ser",
            "value": 167769,
            "range": "± 617",
            "unit": "ns/iter"
          },
          {
            "name": "dbus/fixed_size_array_de",
            "value": 1138953,
            "range": "± 1971",
            "unit": "ns/iter"
          },
          {
            "name": "signature_parse",
            "value": 11017,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "object_path_parse",
            "value": 112,
            "range": "± 4",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}