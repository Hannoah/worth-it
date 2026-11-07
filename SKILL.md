---
name: worth-it
description: 프로젝트 및 도구 기획 단계에서 바퀴 재발명 방지(대안 검증)와 서비스화 실질 ROI를 평가하고, 군더더기 없는 초경량 목적 적합 아키텍처를 처방하는 가치평가 스킬 (Rust Core v0.3.1 Robust & Hardened Edition)
version: 1.0.0
author: kornah
tags: [rust, architecture, evaluation, lean, project-management, automation, dashboard]
---

# ⚖️ worth-it: 프로젝트 가치평가 및 초경량 아키텍처 처방 스킬

새로운 프로젝트, 도구, 자동화 스크립트 아이디어가 떠올랐을 때 **코드를 한 줄도 작성하기 전**에 착수 가치를 냉정하게 판별하고, 오버엔지니어링을 사전에 차단하는 AI 에이전트 스킬입니다.

---

## 🏛️ 핵심 설계 6대 철학

에이전트는 모든 평가와 아키텍처 추천 시 다음 6가지 원칙을 엄격히 고수해야 합니다:

1. **군더더기 없음 (No Bloat):** 불필요한 의존성, 과도한 계층 구조, 복잡한 설정 배제
2. **최적화 (Optimized):** 런타임 자원과 시작 시간(Cold Start)의 최소화
3. **간결함 (Concise / KISS):** 단일 파일, 최소한의 라인 수로 문제 해결
4. **목적 적합성 (Fit for Purpose):** 요구사항에 딱 맞는 형태(CLI/에이전트 도구/정적 HTML) 선택
5. **불필요한 기능 구현 금지 (YAGNI):** 지금 당장 쓰지 않는 기능/확장성 설계 금지
6. **가볍고 확실하게 (Lightweight & Solid):** 메모리/타입 안전성, 1초 내 실행되는 테스트 동반

---

## 🚦 에이전트 작동 프로세스 (Workflow)

사용자가 새로운 프로젝트/도구 아이디어를 제시하면 다음 3단계를 순차적으로 실행합니다:

### Step 1: 3대 킬러 룰 검증 (Hard Filter)
아래 3가지 질문을 던져 즉시 탈락 여부를 판별합니다:
- **대안 조사 여부:** "시중에 나온 오픈소스나 기성 도구 3개를 확인해 보셨습니까?" (없다면 먼저 조사 권고)
- **3의 규칙:** "이 작업을 실제로 3번 이상 반복하며 고통을 겪으셨습니까?" (1회성이면 스크립트화 금지)
- **시간 역전 경고:** "수작업으로 5분이면 끝날 일을 위해 며칠짜리 시스템을 만들고 있지 않습니까?"

### Step 2: 5대 Gate 점수 산출 (100점 만점)
1. **Gate 1. 바퀴 재발명 검증 (25점):** 기존 대안 대비 독창성 / 사내 폐쇄망·보안 등 특수 환경 필수성
2. **Gate 2. 서비스화 실질 ROI (25점):** 반복 사용 빈도, 연간 시간 절감 규모, 타 에이전트(Jarvis) 연계성
3. **Gate 3. 문제의 고통과 시급성 (20점):** Must-have(치명적 비효율) vs Nice-to-have(단순 편의)
4. **Gate 4. 유지보수 TCO (15점):** 종속성 제로(표준 라이브러리) vs 외부 패키지/업데이트 관리 부담
5. **Gate 5. 전략 및 자산성 (15점):** 주력 비즈니스(SAS, AI 에이전트, 오피스 자동화) 정렬도

### Step 3: 의사결정 및 초경량 아키텍처 처방
- **80점 이상 (🟢 GO):** 정식 개발 승인. 단, 군더더기 없는 최소 스택 처방.
- **60~79점 (🟡 SPIKE):** 정식 개발 금지. **1일(또는 2시간) 타임박스 프로토타입**으로만 착수.
- **대안 우수 (🔵 ADOPT):** 직접 개발 금지. 기존 도구/패키지 설치 및 채택 안내.
- **60점 미만 (🔴 NO-GO):** 프로젝트 폴더 생성 금지. 1회성 명령어로 해결하거나 기각.

---

## 🛠️ 목적별 아키텍처 처방 매트릭스

| 프로젝트 목적 | 권장 구현 형태 (Form Factor) | 권장 스택 (Lean Stack) | 절대 금지 기술 (Anti-Bloat) |
|---|---|---|---|
| **초고속 대용량 연산 / CLI** | 단일 바이너리 CLI | Rust (`std` 중심, LTO/Strip 단일 exe) | 런타임 인터프리터, 과도한 의존 크레이트 |
| **Jarvis / MCP 에이전트 도구** | 단일 모듈 도구 / JSON-RPC | Rust 단일 바이너리 or Python 표준 모듈 | 별도 FastAPI/Actix 서버 띄우기, 별도 Docker |
| **오피스 자동화 / 파일 처리** | 단일 파일 CLI | Rust 또는 Python 표준 라이브러리 | `requirements.txt` 패키지 남발, 과도한 계층 구조 |
| **현황판 / 시각화 대시보드** | 더블클릭 실행 단일 HTML | HTML5 + Vanilla JS + 인라인 JSON 데이터 | React, Vue, Next.js, Node.js 빌드 파이프라인 |

---

## 💻 Rust Core 바이너리 실행

```powershell
# 대화형 퀵 마법사 실행
C:\Users\kornah\Work\11_Office_Automation_System\worth-it\target\release\worth-it.exe

# 인터랙티브 HTML 대시보드 생성
C:\Users\kornah\Work\11_Office_Automation_System\worth-it\target\release\worth-it.exe --name "my-tool" --type agent_tool --scores 22 20 18 14 12 --html dashboard.html
```
