# Protocol Buffers Compatibility Policy

`oas.vehicle.v1`은 OAS의 versioned public contract다. Vehicle signal의 출처나 OEM naming은 이 contract에 포함하지 않는다.

## 호환성 규칙

- 배포된 field number는 삭제·재사용·의미 변경하지 않는다.
- 새 상태는 새 field number를 추가하는 additive change로 제공한다.
- 제거가 필요한 field는 `deprecated = true`로 표시하고 number는 예약한다.
- enum의 `*_UNSPECIFIED = 0`은 유지한다. 이미 배포된 enum number의 의미를 바꾸지 않는다.
- signal이 관측되지 않았음을 0이나 `false`와 혼동하지 않도록 선택적 scalar는 `optional`로 표현한다.
- `VehicleState.night_mode`는 선택적 bool이다. `true`와 `false`는 검증된 차량 입력 상태(예: 저빔 ON/OFF)이고, `unset`은 차량이 신호를 제공하지 않았음을 뜻한다.
- breaking change는 새 package major version(예: `oas.vehicle.v2`)으로만 제공하며 migration 문서를 함께 낸다.

## 검증

`buf lint`는 schema style을 확인한다. SDK CI는 `enable-breaking-check: true`로 PR base schema와의 `buf breaking` 검사를 활성화한다. consumer는 SDK Git SHA를 고정하며 generated source를 커밋하지 않는다.

`HmiState`의 freshness와 capability는 Runtime이 소유한다. unknown/unspecified enum은 허용으로 해석하지 않는다. `VehicleState.raw_signals`는 DBC 원래 이름을 보존하는 진단 전용 예외이며 public 제어 계약이 아니다. C++ HMI는 로컬 수신 만료 시 표시와 capability를 닫을 수 있지만, 스스로 허용 상태를 만들지 않는다. Demo는 합성 상태임을 화면에 표시한다.

## Ownership

schema 변경에는 `sdk`, `car` 및 영향 받는 Adapter 담당자의 검토가 필요하다. Vehicle Control command schema는 read-only `VehicleState`와 별도 package로 설계하며 Safety review 없이는 추가하지 않는다.
