# Protocol Buffers Compatibility Policy

`oas.vehicle.v1`은 OAS의 versioned public contract다. Vehicle signal의 출처나 OEM naming은 이 contract에 포함하지 않는다.

## 호환성 규칙

- 배포된 field number는 삭제·재사용·의미 변경하지 않는다.
- 새 상태는 새 field number를 추가하는 additive change로 제공한다.
- 제거가 필요한 field는 `deprecated = true`로 표시하고 number는 예약한다.
- enum의 `*_UNSPECIFIED = 0`은 유지한다. 이미 배포된 enum number의 의미를 바꾸지 않는다.
- signal이 관측되지 않았음을 0이나 `false`와 혼동하지 않도록 선택적 scalar는 `optional`로 표현한다.
- breaking change는 새 package major version(예: `oas.vehicle.v2`)으로만 제공하며 migration 문서를 함께 낸다.

## 검증

`buf lint`는 schema style을 확인한다. PR에서 이전 `main` schema와의 breaking change 검사를 활성화하기 전에는, generated SDK와 consumer를 추가한 뒤 기준 branch와 stable tag를 명시한다.

## Ownership

schema 변경에는 `sdk`, `car` 및 영향 받는 Adapter 담당자의 검토가 필요하다. Vehicle Control command schema는 read-only `VehicleState`와 별도 package로 설계하며 Safety review 없이는 추가하지 않는다.
