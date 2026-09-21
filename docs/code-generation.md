# Code Generation Policy

## Source of truth

`proto/`와 `buf.yaml`만 OAS schema의 source of truth다. generated source는 이 저장소에 commit하지 않는다. 각 consumer는 versioned schema release를 입력으로 자기 언어의 package를 생성·배포한다.

이 정책은 생성물 diff가 schema review를 가리는 일을 막고, Rust/Python/TypeScript/C++ consumer가 각자의 package lifecycle을 갖게 한다.

## Reproducible generation

generation은 Buf v2 config와 version-pinned remote plugin을 사용한다. schema에 언어별 file option을 넣지 않고 `buf.gen.yaml`의 managed mode에서 consumer별 namespace와 output을 설정한다.

```text
schema release
  → consumer-specific buf.gen.yaml
  → buf generate
  → language package test and publish
```

## 언어별 책임

- Rust: Host-side runtime과 `car`가 사용할 protobuf message crate
- Python: simulator, analysis, reverse-engineering 도구
- TypeScript: Web UI와 diagnostics UI
- C++: 기존 Automotive SDK 연동이 필요한 경우에만 사용

각 package의 이름, registry, release automation은 첫 public consumer가 정해질 때 결정한다. `sdk`는 아직 service transport 또는 Vehicle Control command를 생성하지 않는다.

## Compatibility gate

모든 `sdk` Pull Request는 base branch schema를 기준으로 `buf breaking`을 수행한다. wire/source compatibility를 깨는 변경은 새 major package version과 migration 문서가 필요하다.
