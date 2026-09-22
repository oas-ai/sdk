# OAS Protocol Buffers

여기에 `vehicle`, `control`, `can`, `diagnostics`, `system` schema를 추가합니다. API 안정화 전까지는 breaking-change 정책과 package version을 schema별로 명시합니다.

`VehicleState`는 차량의 canonical snapshot이고, `HmiState`는 Runtime이 freshness와 capability 정책 결과를 함께 제공하는 read-only HMI 모델이다. HMI는 `HmiState`의 capability를 재계산하지 않는다.
