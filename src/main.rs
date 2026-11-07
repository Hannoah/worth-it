//! worth-it: 프로젝트 가치평가 및 초경량 아키텍처 처방 엔진 (v0.3.1 Robust & Hardened Edition)
//!
//! 6대 핵심 철학:
//! 1. 군더더기 없음 (No Bloat) - 외부 의존 크레이트 0개 (Zero External Dependencies)
//! 2. 최적화 (Optimized) - LTO, Strip, 330KB 초경량 단일 바이너리
//! 3. 간결함 (Concise / KISS) - 순수 Rust 표준 라이브러리로 완성된 세련된 인터페이스
//! 4. 목적 적합성 (Fit for Purpose) - 도메인 맞춤형 최소 폼팩터 처방
//! 5. 불필요한 기능 구현 금지 (YAGNI) - 과도한 추상화 및 미래 확장성 설계 차단
//! 6. 가볍고 확실하게 (Lightweight & Solid) - 타입/메모리/인코딩 안전성 및 12개 단위 테스트 완비

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[cfg(windows)]
fn init_windows_console() {
    #[link(name = "kernel32")]
    extern "system" {
        fn SetConsoleOutputCP(wCodePageID: u32) -> i32;
    }
    unsafe {
        // UTF-8 코드페이지 (CP 65001) 설정으로 윈도우 콘솔 유니코드 깨짐 방지
        SetConsoleOutputCP(65001);
    }
}

#[cfg(not(windows))]
fn init_windows_console() {}

// ============================================================================
// 1. CORE DOMAIN TYPES & EVALUATION
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    Go,
    Spike,
    Adopt,
    NoGo,
}

impl Decision {
    pub fn badge(&self) -> &'static str {
        match self {
            Decision::Go => "🟢 GO (본격 개발 & 서비스화 승인)",
            Decision::Spike => "🟡 SPIKE (1일 타임박스 프로토타입 검증)",
            Decision::Adopt => "🔵 ADOPT (기존 도구/오픈소스 채택)",
            Decision::NoGo => "🔴 NO-GO (기각 / 스크랩)",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Decision::Go => "GO",
            Decision::Spike => "SPIKE",
            Decision::Adopt => "ADOPT",
            Decision::NoGo => "NO-GO",
        }
    }

    pub fn summary_ko(&self) -> &'static str {
        match self {
            Decision::Go => "높은 가치와 서비스화 효용이 입증되었습니다. 정식 개발 및 서비스화를 추진하세요.",
            Decision::Spike => "잠재력은 있으나 과도한 투자는 금물입니다. 1일 타임박스 프로토타입(PoC)으로 검증하세요.",
            Decision::Adopt => "더 우수한 기존 대안이 이미 존재합니다. 직접 개발하지 말고 채택하세요.",
            Decision::NoGo => "현재로서는 개발 가치(ROI)가 부족합니다. 1회성 명령어로 대체하거나 기각하세요.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
    CliTool,
    AgentTool,
    Dashboard,
    HighPerfCli,
    Script,
}

impl ProjectType {
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "agent_tool" | "agent" | "mcp" => ProjectType::AgentTool,
            "dashboard" | "html" | "ui" => ProjectType::Dashboard,
            "high_perf_cli" | "perf" | "rust" => ProjectType::HighPerfCli,
            "script" => ProjectType::Script,
            _ => ProjectType::CliTool,
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            ProjectType::CliTool => "cli_tool",
            ProjectType::AgentTool => "agent_tool",
            ProjectType::Dashboard => "dashboard",
            ProjectType::HighPerfCli => "high_perf_cli",
            ProjectType::Script => "script",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            ProjectType::CliTool => "초경량 커맨드라인 CLI (오피스 자동화/파일 처리 등)",
            ProjectType::AgentTool => "Jarvis / MCP AI 에이전트 내장 도구",
            ProjectType::Dashboard => "단일 파일 정적 HTML 대시보드 / 시각화",
            ProjectType::HighPerfCli => "고성능/대용량 단일 바이너리 CLI (Rust/Go)",
            ProjectType::Script => "단순 1회성 스크립트",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GateScores {
    pub gate1_alternative: u32,   // 0~25: 바퀴 재발명 검증
    pub gate2_servitization: u32, // 0~25: 서비스화 실질 ROI
    pub gate3_urgency: u32,       // 0~20: 고통과 시급성
    pub gate4_tco: u32,           // 0~15: 유지보수 TCO
    pub gate5_strategy: u32,      // 0~15: 전략 및 자산성
}

impl GateScores {
    /// 배점 범위를 초과하지 않도록 안전하게 클램핑된 스코어 생성
    pub fn new_clamped(g1: u32, g2: u32, g3: u32, g4: u32, g5: u32) -> Self {
        Self {
            gate1_alternative: g1.min(25),
            gate2_servitization: g2.min(25),
            gate3_urgency: g3.min(20),
            gate4_tco: g4.min(15),
            gate5_strategy: g5.min(15),
        }
    }

    pub fn total(&self) -> u32 {
        self.gate1_alternative
            + self.gate2_servitization
            + self.gate3_urgency
            + self.gate4_tco
            + self.gate5_strategy
    }
}

#[derive(Debug, Clone)]
pub struct ArchitecturePrescription {
    pub form_factor: &'static str,
    pub recommended_stack: &'static str,
    pub forbidden_bloat: Vec<&'static str>,
    pub lean_rules: Vec<&'static str>,
    pub estimated_files: &'static str,
    pub next_action: &'static str,
}

#[derive(Debug, Clone)]
pub struct ValuationResult {
    pub project_name: String,
    pub scores: GateScores,
    pub decision: Decision,
    pub decision_reason: String,
    pub prescription: ArchitecturePrescription,
    pub project_type: ProjectType,
}

pub fn evaluate(
    project_name: &str,
    scores: GateScores,
    project_type: ProjectType,
    performance_critical: bool,
    existing_alternative: Option<&str>,
) -> ValuationResult {
    let total = scores.total();

    // 1. 의사결정 로직
    let (decision, reason) = if scores.gate1_alternative <= 5 && existing_alternative.is_some() {
        let alt = existing_alternative.unwrap();
        (
            Decision::Adopt,
            format!("더 우수한 기존 대안({alt})이 이미 존재합니다. 직접 개발하지 말고 채택하세요."),
        )
    } else if total >= 80 {
        (Decision::Go, Decision::Go.summary_ko().to_string())
    } else if total >= 60 {
        (Decision::Spike, Decision::Spike.summary_ko().to_string())
    } else {
        (Decision::NoGo, Decision::NoGo.summary_ko().to_string())
    };

    // 2. 초경량 아키텍처 처방 (6대 원칙 기반)
    let prescription = prescribe_architecture(project_type, performance_critical, decision);

    ValuationResult {
        project_name: project_name.to_string(),
        scores,
        decision,
        decision_reason: reason,
        prescription,
        project_type,
    }
}

pub fn prescribe_architecture(
    project_type: ProjectType,
    performance_critical: bool,
    decision: Decision,
) -> ArchitecturePrescription {
    if decision == Decision::Adopt {
        return ArchitecturePrescription {
            form_factor: "기성 도구 채택 (No-Code / Package Install)",
            recommended_stack: "기존 오픈소스 / 패키지 매니저(cargo, winget, pip)",
            forbidden_bloat: vec!["자체 신규 개발", "래퍼(Wrapper) 스크립트 남발", "불필요한 설정 파일"],
            lean_rules: vec![
                "새로 코드를 짜지 말고 기존 도구의 CLI 옵션을 최대한 활용할 것",
                "팀 공유 필요 시 설치 명령어 1줄(README)만 문서화할 것",
            ],
            estimated_files: "0개 (설치 및 설정만 진행)",
            next_action: "winget install 또는 cargo/pip으로 기존 도구를 설치하여 파이프라인에 편입하세요.",
        };
    }

    if decision == Decision::NoGo {
        return ArchitecturePrescription {
            form_factor: "착수 금지 (No-Build)",
            recommended_stack: "PowerShell 1줄 명령어 / 수작업 유지",
            forbidden_bloat: vec!["프로젝트 폴더 생성", "모듈화", "프레임워크 도입", "장기 계획 수립"],
            lean_rules: vec![
                "필요할 때 터미널에서 일회성 명령어로 해결할 것",
                "실제 고통이 3회 이상 누적되기 전까지 신규 폴더 생성 금지",
            ],
            estimated_files: "0개",
            next_action: "착수를 기각하고 현재 작업에 집중하세요. 필요시 단발성 명령어로 처리합니다.",
        };
    }

    if performance_critical || project_type == ProjectType::HighPerfCli {
        return ArchitecturePrescription {
            form_factor: "초고속 단일 바이너리 CLI (Single Binary)",
            recommended_stack: "Rust (외부 크레이트 최소화, LTO/Strip 적용 단일 exe)",
            forbidden_bloat: vec![
                "불필요한 무거운 의존 크레이트 남발",
                "인터프리터 언어 런타임",
                "복잡한 비동기(tokio) 남발 (동기 I/O로 충분하면 async 배제)",
            ],
            lean_rules: vec![
                "설치/런타임 의존성 없는 단일 .exe 배포",
                "`lto = true`, `opt-level = \"z\"`, `strip = true`로 바이너리 다이어트",
                "메모리 제로 카피 및 초고속 시작 시간(Cold Start < 1ms) 달성",
            ],
            estimated_files: "단 2~3개 파일 (src/main.rs, Cargo.toml, README.md)",
            next_action: "cargo new --bin 으로 시작하고, 불필요한 크레이트 추가 없이 std 중심으로 작성하세요.",
        };
    }

    match project_type {
        ProjectType::AgentTool => ArchitecturePrescription {
            form_factor: "Jarvis / MCP 에이전트 내장 도구 모듈",
            recommended_stack: "Rust 단일 바이너리 (stdin/stdout JSON-RPC) 또는 Python 표준 모듈",
            forbidden_bloat: vec![
                "독립 웹 서버(FastAPI/Actix) 띄우기",
                "별도 Docker 컨테이너",
                "복잡한 외부 DB 연동",
            ],
            lean_rules: vec![
                "독립 데몬 서버 대신 stdin/stdout 표준 스트림으로 에이전트와 직접 결합",
                "입출력은 표준 JSON 스키마로 최소화",
                "예외 발생 시 크래시 없이 명확한 JSON 에러 메시지 반환",
            ],
            estimated_files: "1개 파일 (도구 소스 1개, 100~200줄)",
            next_action: "Jarvis custom_tools/ 디렉터리에 단일 모듈로 추가하고 audit 로깅을 연동하세요.",
        },
        ProjectType::Dashboard => ArchitecturePrescription {
            form_factor: "단일 정적 HTML 파일 (Self-Contained Single File)",
            recommended_stack: "순수 HTML5 + Vanilla JS + 인라인 JSON 데이터",
            forbidden_bloat: vec![
                "Node.js / npm 생태계 (React, Next.js, Vite)",
                "별도 백엔드 API 서버",
                "무거운 빌드/번들링 파이프라인",
            ],
            lean_rules: vec![
                "더블클릭하면 브라우저에서 즉시 열리는 단일 HTML 파일로 완결",
                "외부 CDN 의존성 배제 또는 최소화",
                "CSS 프레임워크 없이 심플 모던 CSS 50줄로 끝낼 것",
            ],
            estimated_files: "단일 파일 1개 (`dashboard.html`)",
            next_action: "단일 HTML 파일에 데이터 스크립트를 인라인으로 임베딩하여 로컬 실행을 보장하세요.",
        },
        ProjectType::Script => ArchitecturePrescription {
            form_factor: "초경량 1회성 스크립트",
            recommended_stack: "PowerShell (.ps1) 또는 Python 단일 파일",
            forbidden_bloat: vec!["프로젝트화", "클래스/디자인 패턴", "설정 파일"],
            lean_rules: vec!["스크립트 1개 50줄 이내로 작성 후 추가 개발 금지"],
            estimated_files: "1개 파일",
            next_action: "PowerShell 한 줄 스크립트나 단일 .ps1 파일로 작성 후 즉시 실행하세요.",
        },
        ProjectType::CliTool => ArchitecturePrescription {
            form_factor: "초경량 CLI 유틸리티",
            recommended_stack: "Rust 표준 라이브러리(Zero-dep) 또는 Python 표준 라이브러리",
            forbidden_bloat: vec![
                "외부 패키지/크레이트 의존성 3개 이상 추가 금지",
                "과도한 계층 분리(Controller, Service, DAO 금지)",
                "복잡한 설정 파일 남발 (.yaml, .toml, .ini 금지)",
            ],
            lean_rules: vec![
                "단 하나의 소스 파일로 핵심 기능 완결 (Single Responsibility)",
                "테스트 1개만 작성하여 1초 이내 검증 완료",
                "터미널 출력은 읽기 쉬운 Markdown 또는 Plain Text 사용",
            ],
            estimated_files: "코드 1개 + 테스트 1개 + README 1개",
            next_action: "표준 라이브러리만으로 단일 진입점 소스 파일을 작성하고 즉시 테스트하세요.",
        },
        ProjectType::HighPerfCli => unreachable!(),
    }
}

// ============================================================================
// 2. TERMINAL UI & ANSI STYLING (Zero External Dependencies)
// ============================================================================

pub fn render_gauge_modern(val: u32, max: u32, width: usize) -> String {
    let ratio = if max == 0 { 0.0 } else { (val as f64 / max as f64).clamp(0.0, 1.0) };
    let filled = (ratio * width as f64).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("{}{}", "▰".repeat(filled), "▱".repeat(empty))
}

#[derive(Clone, Copy)]
pub struct TermUi {
    pub enabled: bool,
}

impl TermUi {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    pub fn green(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[32m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn bright_green(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[92m\x1b[1m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn yellow(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[33m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn red(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[31m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn cyan(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[36m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn bold(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[1m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn dim(&self, s: &str) -> String {
        if self.enabled { format!("\x1b[90m{}\x1b[0m", s) } else { s.to_string() }
    }

    pub fn score_styled(&self, score: u32) -> String {
        if score >= 80 {
            self.bright_green(&format!("{}점", score))
        } else if score >= 60 {
            self.yellow(&format!("{}점", score))
        } else {
            self.red(&format!("{}점", score))
        }
    }

    pub fn decision_styled(&self, d: Decision) -> String {
        match d {
            Decision::Go => self.bright_green(&self.bold(d.badge())),
            Decision::Spike => self.yellow(&self.bold(d.badge())),
            Decision::Adopt => self.cyan(&self.bold(d.badge())),
            Decision::NoGo => self.red(&self.bold(d.badge())),
        }
    }

    pub fn render_banner(&self, title: &str) {
        let width = 64;
        let line = "─".repeat(width);
        println!("{}", self.cyan(&format!("╭{}╮", line)));
        println!("│ {:<62} │", self.bold(title));
        println!("{}", self.cyan(&format!("╰{}╯", line)));
    }
}

/// 터미널 전용 미려한 대시보드 카드 렌더링
pub fn render_terminal_dashboard(result: &ValuationResult, ui: &TermUi) -> String {
    let p = &result.prescription;
    let s = &result.scores;
    let total = s.total();
    let gauge = render_gauge_modern(total, 100, 20);

    let mut out = String::new();
    let border = ui.cyan(&"─".repeat(66));

    out.push_str(&format!("╭{}╮\n", border));
    out.push_str(&format!("│  ⚖️  {:<58} │\n", ui.bold(&format!("worth-it 판정 대시보드: {}", result.project_name))));
    out.push_str(&format!("├{}┤\n", border));
    out.push_str(&format!("│  🎯 {:<12} : {:<47} │\n", ui.bold("최종 판정"), ui.decision_styled(result.decision)));
    out.push_str(&format!("│  📊 {:<12} : {} / 100점  {:<24} │\n", ui.bold("가치 점수"), ui.score_styled(total), ui.green(&gauge)));
    out.push_str(&format!("│  💡 {:<12} : {:<47} │\n", ui.bold("판정 사유"), result.decision_reason));
    out.push_str(&format!("├{}┤\n", border));
    out.push_str(&format!("│  🛠️  {:<11} : {:<47} │\n", ui.bold("권장 형태"), ui.yellow(p.form_factor)));
    out.push_str(&format!("│  📦 {:<12} : {:<47} │\n", ui.bold("권장 스택"), p.recommended_stack));
    out.push_str(&format!("│  📁 {:<12} : {:<47} │\n", ui.bold("예상 규모"), p.estimated_files));
    out.push_str(&format!("├{}┤\n", border));
    out.push_str(&format!("│  🚫 {} : {:<45} │\n", ui.red("금지 기술"), ui.dim(&p.forbidden_bloat.join(", "))));
    out.push_str(&format!("│  🚀 {} : {:<45} │\n", ui.green("다음 액션"), ui.bold(p.next_action)));
    out.push_str(&format!("╰{}╯\n", border));

    // Gate별 미니 카드 표
    out.push_str("\n┌────────┬──────────────────────┬──────┬──────┬────────────────────────┐\n");
    out.push_str("│  Gate  │ 평가 항목            │ 배점 │ 득점 │ 진척 게이지            │\n");
    out.push_str("├────────┼──────────────────────┼──────┼──────┼────────────────────────┤\n");

    let gates = [
        ("Gate 1", "바퀴 재발명 검증", 25, s.gate1_alternative),
        ("Gate 2", "서비스화 실질 ROI", 25, s.gate2_servitization),
        ("Gate 3", "문제 고통/시급성", 20, s.gate3_urgency),
        ("Gate 4", "유지보수 TCO", 15, s.gate4_tco),
        ("Gate 5", "전략 및 자산성", 15, s.gate5_strategy),
    ];

    for (g_id, g_name, max_p, actual_p) in gates {
        let g_bar = render_gauge_modern(actual_p, max_p, 12);
        let pct = (actual_p as f64 / max_p as f64 * 100.0).round() as u32;
        out.push_str(&format!(
            "│ {:<6} │ {:<20} │ {:>4} │ {:>4} │ {} {:>3}%  │\n",
            ui.bold(g_id),
            g_name,
            max_p,
            actual_p,
            ui.cyan(&g_bar),
            pct
        ));
    }
    out.push_str("├────────┼──────────────────────┼──────┼──────┼────────────────────────┤\n");
    out.push_str(&format!(
        "│ {:<6} │ {:<20} │ {:>4} │ {:>4} │ {} {:>3}%  │\n",
        ui.bold("합  계"),
        ui.bold("최종 가치 점수"),
        100,
        total,
        ui.bright_green(&render_gauge_modern(total, 100, 12)),
        total
    ));
    out.push_str("└────────┴──────────────────────┴──────┴──────┴────────────────────────┘\n");

    out
}

// ============================================================================
// 3. FORMATTERS (MARKDOWN, SECURE JSON & XSS-SAFE HTML DASHBOARD)
// ============================================================================

pub fn format_markdown_report(result: &ValuationResult) -> String {
    let p = &result.prescription;
    let s = &result.scores;
    let bar = render_gauge_modern(s.total(), 100, 20);

    let mut out = String::new();
    out.push_str(&format!("# ⚖️ worth-it 가치평가 리포트: `{}`\n\n", result.project_name));
    out.push_str(&format!("> **최종 판정:** {}\n", result.decision.badge()));
    out.push_str(&format!("> **가치평가 총점:** **{}점** / 100점 `{}`\n\n", s.total(), bar));
    out.push_str(&format!("**판정 사유:** {}\n\n", result.decision_reason));
    out.push_str("---\n\n");

    out.push_str("## 1. 📊 5대 Gate 세부 점수\n\n");
    out.push_str("| Gate | 평가 항목 | 배점 | 득점 | 진척도 | 핵심 검증 포인트 |\n");
    out.push_str("|:---:|---|:---:|:---:|:---:|---|\n");
    out.push_str(&format!("| **Gate 1** | **바퀴 재발명 검증** | 25 | **{}** | `{}` | 기존 대안 대비 차별성 / 사내 특화 필수성 |\n", s.gate1_alternative, render_gauge_modern(s.gate1_alternative, 25, 10)));
    out.push_str(&format!("| **Gate 2** | **서비스화 실질 ROI** | 25 | **{}** | `{}` | 반복 사용 빈도 및 수작업 시간 절감 규모 |\n", s.gate2_servitization, render_gauge_modern(s.gate2_servitization, 25, 10)));
    out.push_str(&format!("| **Gate 3** | **문제의 고통/시급성** | 20 | **{}** | `{}` | Must-have vs Nice-to-have |\n", s.gate3_urgency, render_gauge_modern(s.gate3_urgency, 20, 10)));
    out.push_str(&format!("| **Gate 4** | **유지보수 TCO** | 15 | **{}** | `{}` | 종속성 제로 / 관리 부채 최소화 |\n", s.gate4_tco, render_gauge_modern(s.gate4_tco, 15, 10)));
    out.push_str(&format!("| **Gate 5** | **전략 및 자산성** | 15 | **{}** | `{}` | 주력 비즈니스(SAS, Jarvis, 자동화) 연계도 |\n", s.gate5_strategy, render_gauge_modern(s.gate5_strategy, 15, 10)));
    out.push_str(&format!("| **합계** | **최종 가치 점수** | **100** | **{}** | `{}` | 80점 이상 GO / 60~79점 SPIKE / 60점 미만 NO-GO |\n\n", s.total(), bar));
    out.push_str("---\n\n");

    out.push_str("## 2. 🛠️ 목적 적합 초경량 아키텍처 처방전\n\n");
    out.push_str("> **설계 6대 원칙:** 군더더기 없음 · 최적화 · 간결함 · 목적 적합성 · YAGNI · 가볍고 확실하게\n\n");
    out.push_str(&format!("- **추천 구현 형태:** `{}`\n", p.form_factor));
    out.push_str(&format!("- **권장 기술 스택:** `{}`\n", p.recommended_stack));
    out.push_str(&format!("- **예상 파일 규모:** `{}`\n\n", p.estimated_files));

    out.push_str("### 🚫 금지 기술 (Anti-Bloat Checklist)\n");
    out.push_str("불필요한 복잡성을 유발하므로 **절대 도입하지 마십시오:**\n");
    for b in &p.forbidden_bloat {
        out.push_str(&format!("- ❌ {}\n", b));
    }

    out.push_str("\n### 🎯 핵심 린(Lean) 수칙\n");
    for r in &p.lean_rules {
        out.push_str(&format!("- [x] {}\n", r));
    }

    out.push_str("\n### 🚀 다음 액션 (Next Action)\n");
    out.push_str(&format!("> {}\n\n", p.next_action));

    out
}

/// RFC 8259 완벽 준수 JSON 이스케이프 (제어문자 \u0000~\u001f 완전 처리)
pub fn escape_json(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\x08' => out.push_str("\\b"),
            '\x0C' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => out.push(c),
        }
    }
    out
}

/// HTML XSS 및 마크업 깨짐 방지 이스케이프
pub fn escape_html(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            _ => out.push(c),
        }
    }
    out
}

pub fn format_json_report(result: &ValuationResult) -> String {
    let p = &result.prescription;
    let s = &result.scores;

    let mut forbidden_json = String::new();
    for (i, b) in p.forbidden_bloat.iter().enumerate() {
        if i > 0 {
            forbidden_json.push_str(", ");
        }
        forbidden_json.push_str(&format!("\"{}\"", escape_json(b)));
    }

    let mut rules_json = String::new();
    for (i, r) in p.lean_rules.iter().enumerate() {
        if i > 0 {
            rules_json.push_str(", ");
        }
        rules_json.push_str(&format!("\"{}\"", escape_json(r)));
    }

    format!(
        r#"{{
  "project_name": "{}",
  "decision": "{}",
  "total_score": {},
  "decision_reason": "{}",
  "project_type": "{}",
  "scores": {{
    "gate1_alternative": {},
    "gate2_servitization": {},
    "gate3_urgency": {},
    "gate4_tco": {},
    "gate5_strategy": {}
  }},
  "prescription": {{
    "form_factor": "{}",
    "recommended_stack": "{}",
    "estimated_files": "{}",
    "forbidden_bloat": [{}],
    "lean_rules": [{}],
    "next_action": "{}"
  }}
}}"#,
        escape_json(&result.project_name),
        result.decision.code(),
        s.total(),
        escape_json(&result.decision_reason),
        result.project_type.code(),
        s.gate1_alternative,
        s.gate2_servitization,
        s.gate3_urgency,
        s.gate4_tco,
        s.gate5_strategy,
        escape_json(p.form_factor),
        escape_json(p.recommended_stack),
        escape_json(p.estimated_files),
        forbidden_json,
        rules_json,
        escape_json(p.next_action)
    )
}

/// 완전 독립형(Self-Contained) 다크 테마 HTML 대시보드 생성기 (XSS 방어 완료)
pub fn format_html_report(result: &ValuationResult) -> String {
    let p = &result.prescription;
    let s = &result.scores;
    let total = s.total();

    let (badge_color, badge_text) = match result.decision {
        Decision::Go => ("#10b981", "GO · 본격 개발 & 서비스화 승인"),
        Decision::Spike => ("#f59e0b", "SPIKE · 1일 타임박스 PoC 검증"),
        Decision::Adopt => ("#06b6d4", "ADOPT · 기존 도구 채택 권고"),
        Decision::NoGo => ("#ef4444", "NO-GO · 착수 기각"),
    };

    let md_raw = format_markdown_report(result);
    // script 태그 내 안전 주입을 위해 </script> 이스케이프
    let md_escaped = escape_json(&md_raw).replace("</script>", "<\\/script>");

    let mut forbidden_html = String::new();
    for b in &p.forbidden_bloat {
        forbidden_html.push_str(&format!(
            r#"<span class="badge badge-forbidden">❌ {}</span>"#,
            escape_html(b)
        ));
    }

    let mut rules_html = String::new();
    for r in &p.lean_rules {
        rules_html.push_str(&format!(
            r#"<div class="rule-item"><span class="check">✓</span><span>{}</span></div>"#,
            escape_html(r)
        ));
    }

    let g1_pct = (s.gate1_alternative * 100 / 25).min(100);
    let g2_pct = (s.gate2_servitization * 100 / 25).min(100);
    let g3_pct = (s.gate3_urgency * 100 / 20).min(100);
    let g4_pct = (s.gate4_tco * 100 / 15).min(100);
    let g5_pct = (s.gate5_strategy * 100 / 15).min(100);

    // SVG 원형 차트 둘레: 반지름 42, 둘레 2 * pi * 42 ≈ 263.89
    let circumference = 263.89;
    let stroke_offset = circumference * (1.0 - (total as f64 / 100.0).clamp(0.0, 1.0));

    format!(r##"<!DOCTYPE html>
<html lang="ko">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>worth-it 대시보드 - {project_name}</title>
  <style>
    :root {{
      --bg: #0f172a;
      --card-bg: #1e293b;
      --card-border: #334155;
      --text: #f8fafc;
      --text-muted: #94a3b8;
      --accent: #38bdf8;
      --badge-color: {badge_color};
    }}
    * {{ box-sizing: border-box; margin: 0; padding: 0; }}
    body {{
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
      background: var(--bg);
      color: var(--text);
      padding: 2rem 1rem;
      display: flex;
      justify-content: center;
    }}
    .container {{
      max-width: 900px;
      width: 100%;
      display: flex;
      flex-direction: column;
      gap: 1.5rem;
    }}
    .header {{
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding-bottom: 1rem;
      border-bottom: 1px solid var(--card-border);
    }}
    .brand {{
      display: flex;
      align-items: center;
      gap: 0.5rem;
      font-size: 1.25rem;
      font-weight: 700;
      color: var(--accent);
    }}
    .tagline {{ font-size: 0.85rem; color: var(--text-muted); }}
    .card {{
      background: var(--card-bg);
      border: 1px solid var(--card-border);
      border-radius: 12px;
      padding: 1.5rem;
      box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.3);
    }}
    .hero-card {{
      display: grid;
      grid-template-columns: 140px 1fr;
      gap: 1.5rem;
      align-items: center;
    }}
    @media (max-width: 600px) {{
      .hero-card {{ grid-template-columns: 1fr; text-align: center; }}
      .hero-gauge {{ margin: 0 auto; }}
    }}
    .hero-gauge {{
      width: 120px;
      height: 120px;
      position: relative;
    }}
    .hero-gauge svg {{
      transform: rotate(-90deg);
      width: 100%;
      height: 100%;
    }}
    .hero-gauge-score {{
      position: absolute;
      top: 50%;
      left: 50%;
      transform: translate(-50%, -50%);
      font-size: 1.75rem;
      font-weight: 800;
      color: var(--badge-color);
    }}
    .badge {{
      display: inline-block;
      padding: 0.35rem 0.75rem;
      border-radius: 9999px;
      font-weight: 700;
      font-size: 0.85rem;
      background: rgba(255, 255, 255, 0.08);
      color: var(--badge-color);
      border: 1px solid var(--badge-color);
      margin-bottom: 0.5rem;
    }}
    .project-title {{ font-size: 1.75rem; font-weight: 800; margin-bottom: 0.25rem; }}
    .reason-text {{ color: var(--text-muted); font-size: 0.95rem; line-height: 1.5; }}
    .section-title {{
      font-size: 1.1rem;
      font-weight: 700;
      margin-bottom: 1rem;
      display: flex;
      align-items: center;
      gap: 0.5rem;
    }}
    .gate-grid {{ display: flex; flex-direction: column; gap: 0.75rem; }}
    .gate-row {{
      display: grid;
      grid-template-columns: 90px 1fr 60px;
      align-items: center;
      gap: 1rem;
    }}
    .gate-label {{ font-size: 0.85rem; font-weight: 600; }}
    .bar-bg {{
      background: #334155;
      height: 10px;
      border-radius: 999px;
      overflow: hidden;
    }}
    .bar-fill {{
      height: 100%;
      border-radius: 999px;
      background: var(--accent);
      transition: width 0.5f ease;
    }}
    .gate-pts {{ font-size: 0.85rem; text-align: right; color: var(--accent); font-weight: 700; }}
    .presc-grid {{
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 1rem;
      margin-bottom: 1rem;
    }}
    @media (max-width: 600px) {{ .presc-grid {{ grid-template-columns: 1fr; }} }}
    .presc-item {{ background: rgba(0,0,0,0.2); padding: 0.85rem; border-radius: 8px; }}
    .presc-label {{ font-size: 0.75rem; color: var(--text-muted); text-transform: uppercase; margin-bottom: 0.25rem; }}
    .presc-val {{ font-weight: 600; font-size: 0.95rem; }}
    .badge-forbidden {{
      background: rgba(239, 68, 68, 0.15);
      border-color: #ef4444;
      color: #fca5a5;
      margin-right: 0.4rem;
      margin-bottom: 0.4rem;
    }}
    .rule-item {{
      display: flex;
      align-items: center;
      gap: 0.6rem;
      font-size: 0.9rem;
      padding: 0.4rem 0;
      border-bottom: 1px solid rgba(255,255,255,0.05);
    }}
    .rule-item .check {{ color: #10b981; font-weight: bold; }}
    .next-box {{
      background: rgba(16, 185, 129, 0.1);
      border-left: 4px solid #10b981;
      padding: 1rem;
      border-radius: 4px;
      margin-top: 1rem;
      font-size: 0.95rem;
    }}
    .btn-copy {{
      background: var(--accent);
      color: #0f172a;
      border: none;
      padding: 0.6rem 1.2rem;
      border-radius: 6px;
      font-weight: 700;
      cursor: pointer;
      display: inline-flex;
      align-items: center;
      gap: 0.5rem;
      transition: opacity 0.2s;
    }}
    .btn-copy:hover {{ opacity: 0.9; }}
    .footer {{ text-align: center; font-size: 0.8rem; color: var(--text-muted); margin-top: 1rem; }}
  </style>
</head>
<body>
  <div class="container">
    <div class="header">
      <div class="brand">⚖️ worth-it <span class="tagline">· Lean Architecture Engine</span></div>
      <button class="btn-copy" onclick="copyMarkdown()">📋 마크다운 복사</button>
    </div>

    <!-- Hero Card -->
    <div class="card hero-card">
      <div class="hero-gauge">
        <svg viewBox="0 0 100 100">
          <circle cx="50" cy="50" r="42" fill="none" stroke="#334155" stroke-width="8"></circle>
          <circle cx="50" cy="50" r="42" fill="none" stroke="{badge_color}" stroke-width="8"
                  stroke-dasharray="{circumference}" stroke-dashoffset="{stroke_offset}" stroke-linecap="round"></circle>
        </svg>
        <div class="hero-gauge-score">{total}</div>
      </div>
      <div>
        <span class="badge">{badge_text}</span>
        <h1 class="project-title">{project_name}</h1>
        <p class="reason-text">{decision_reason}</p>
      </div>
    </div>

    <!-- 5 Gates Breakdown -->
    <div class="card">
      <h2 class="section-title">📊 5대 Gate 세부 평가</h2>
      <div class="gate-grid">
        <div class="gate-row">
          <span class="gate-label">G1. 대안 검증</span>
          <div class="bar-bg"><div class="bar-fill" style="width: {g1_pct}%;"></div></div>
          <span class="gate-pts">{g1} / 25</span>
        </div>
        <div class="gate-row">
          <span class="gate-label">G2. 서비스 ROI</span>
          <div class="bar-bg"><div class="bar-fill" style="width: {g2_pct}%;"></div></div>
          <span class="gate-pts">{g2} / 25</span>
        </div>
        <div class="gate-row">
          <span class="gate-label">G3. 문제 시급성</span>
          <div class="bar-bg"><div class="bar-fill" style="width: {g3_pct}%;"></div></div>
          <span class="gate-pts">{g3} / 20</span>
        </div>
        <div class="gate-row">
          <span class="gate-label">G4. 유지보수 TCO</span>
          <div class="bar-bg"><div class="bar-fill" style="width: {g4_pct}%;"></div></div>
          <span class="gate-pts">{g4} / 15</span>
        </div>
        <div class="gate-row">
          <span class="gate-label">G5. 전략 정렬도</span>
          <div class="bar-bg"><div class="bar-fill" style="width: {g5_pct}%;"></div></div>
          <span class="gate-pts">{g5} / 15</span>
        </div>
      </div>
    </div>

    <!-- Architecture Prescription -->
    <div class="card">
      <h2 class="section-title">🛠️ 목적 적합 초경량 아키텍처 처방전</h2>
      <div class="presc-grid">
        <div class="presc-item">
          <div class="presc-label">추천 폼팩터</div>
          <div class="presc-val">{form_factor}</div>
        </div>
        <div class="presc-item">
          <div class="presc-label">권장 기술 스택</div>
          <div class="presc-val">{recommended_stack}</div>
        </div>
        <div class="presc-item">
          <div class="presc-label">예상 파일 규모</div>
          <div class="presc-val">{estimated_files}</div>
        </div>
        <div class="presc-item">
          <div class="presc-label">프로젝트 분류</div>
          <div class="presc-val">{project_type}</div>
        </div>
      </div>

      <div style="margin-bottom: 1rem;">
        <div class="presc-label" style="margin-bottom: 0.5rem;">🚫 금지 기술 (Anti-Bloat)</div>
        <div>{forbidden_html}</div>
      </div>

      <div style="margin-bottom: 1rem;">
        <div class="presc-label" style="margin-bottom: 0.5rem;">🎯 핵심 린(Lean) 수칙</div>
        <div>{rules_html}</div>
      </div>

      <div class="next-box">
        <strong>🚀 다음 액션:</strong> {next_action}
      </div>
    </div>

    <div class="footer">
      Generated by worth-it (Zero-Dependency Rust Core) · 6대 린 엔지니어링 철학 준수
    </div>
  </div>

  <script>
    const markdownText = `{md_escaped}`;
    function copyMarkdown() {{
      navigator.clipboard.writeText(markdownText).then(() => {{
        alert("✅ 마크다운 리포트가 클립보드에 복사되었습니다!");
      }}).catch(() => {{
        alert("복사 실패. 브라우저 권한을 확인하세요.");
      }});
    }}
  </script>
</body>
</html>
"##,
        project_name = escape_html(&result.project_name),
        badge_color = badge_color,
        badge_text = badge_text,
        total = total,
        circumference = circumference,
        stroke_offset = stroke_offset,
        decision_reason = escape_html(&result.decision_reason),
        g1 = s.gate1_alternative,
        g2 = s.gate2_servitization,
        g3 = s.gate3_urgency,
        g4 = s.gate4_tco,
        g5 = s.gate5_strategy,
        g1_pct = g1_pct,
        g2_pct = g2_pct,
        g3_pct = g3_pct,
        g4_pct = g4_pct,
        g5_pct = g5_pct,
        form_factor = escape_html(p.form_factor),
        recommended_stack = escape_html(p.recommended_stack),
        estimated_files = escape_html(p.estimated_files),
        project_type = escape_html(result.project_type.label()),
        forbidden_html = forbidden_html,
        rules_html = rules_html,
        next_action = escape_html(p.next_action),
        md_escaped = md_escaped
    )
}

// ============================================================================
// 4. INTERACTIVE CLI RUNNER (Quick Wizard & Live Feedback)
// ============================================================================

fn prompt_line(msg: &str) -> String {
    print!("{}", msg);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input.trim().to_string()
}

fn prompt_number(msg: &str, min: u32, max: u32, default: u32) -> u32 {
    loop {
        print!("{} [{}~{}, 기본 {}]: ", msg, min, max, default);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            return default;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return default;
        }
        match trimmed.parse::<u32>() {
            Ok(v) if v >= min && v <= max => return v,
            _ => eprintln!("  ⚠️ {}부터 {} 사이의 숫자를 입력하세요.", min, max),
        }
    }
}

fn estimate_interim_status(current_score: u32) -> &'static str {
    if current_score >= 80 {
        "🟢 GO 구간 진입"
    } else if current_score >= 60 {
        "🟡 SPIKE 구간"
    } else {
        "🔴 아직 점수 미달"
    }
}

fn ask_gate_quick(
    ui: &TermUi,
    step: u32,
    title: &str,
    max_pts: u32,
    opt1: (&str, u32),
    opt2: (&str, u32),
    opt3: (&str, u32),
    running_total: u32,
) -> u32 {
    println!("\n╭─────────────────────────────────────────────────────────────╮");
    println!("│ {} {} {:<23} │", ui.bold(&format!("Step [{}/5]", step)), ui.cyan(title), ui.dim(&format!("(배점: {}점)", max_pts)));
    println!("╰─────────────────────────────────────────────────────────────╯");
    println!("  1) {} {}", ui.bold("상(High):"), opt1.0);
    println!("  2) {} {}", ui.bold("중(Med) :"), opt2.0);
    println!("  3) {} {}", ui.bold("하(Low) :"), opt3.0);
    println!("  4) 직접 점수 입력 (0~{})", max_pts);

    let sel = prompt_number("  -> 선택", 1, 4, 1);
    let pts = match sel {
        1 => opt1.1,
        2 => opt2.1,
        3 => opt3.1,
        _ => prompt_number("  -> 직접 점수 입력", 0, max_pts, opt2.1),
    };

    let new_total = running_total + pts;
    println!("  {} 누적 {}점 | {}", ui.green("✓ 반영:"), new_total, ui.dim(estimate_interim_status(new_total)));
    pts
}

fn run_interactive(ui: &TermUi, preset_name: Option<String>) {
    ui.render_banner("⚖️ worth-it v0.3.1 · 프로젝트 가치평가 및 아키텍처 처방기");
    println!("{}\n", ui.dim("철학: 군더더기 없음 · 최적화 · 간결함 · 목적 적합성 · YAGNI · 가볍고 확실하게"));

    let name = if let Some(n) = preset_name {
        println!("📌 프로젝트 이름: {}", ui.bold(&n));
        n
    } else {
        let input = prompt_line("📌 프로젝트 / 도구 아이디어 이름: ");
        if input.is_empty() { "unnamed-project".to_string() } else { input }
    };

    println!("\n평가 모드를 선택하세요:");
    println!("  1) {} 30초 퀵 마법사 (상/중/하 빠른 선택 및 실시간 피드백) [권장]", ui.bright_green("[★]"));
    println!("  2) 상세 점수 직접 입력 모드");
    let mode_sel = prompt_number("선택", 1, 2, 1);

    let (g1, alt_name, g2, g3, g4, g5) = if mode_sel == 1 {
        // Quick Wizard Mode with running score feedback
        let mut running = 0;
        let g1 = ask_gate_quick(
            ui, 1, "바퀴 재발명 검증", 25,
            ("기존 대안 없음 (사내 환경/보안/독창적 로직 필수)", 25),
            ("대안 있으나 너무 무거움 (5% 핵심 기능만 초경량 필요)", 15),
            ("이미 훌륭한 오픈소스가 있음 (직접 만들기 비추천)", 5),
            running,
        );
        running += g1;

        let alt = if g1 <= 10 {
            let a = prompt_line("     기존에 존재하는 대안 도구 이름 (없으면 엔터): ");
            if a.is_empty() { None } else { Some(a) }
        } else {
            None
        };

        let g2 = ask_gate_quick(
            ui, 2, "서비스화 실질 ROI", 25,
            ("반복 빈도 극상, Jarvis/동료 확산 가능, 연 수십 시간 절감", 25),
            ("월 1~2회 쓰이지만 수작업 대비 확실한 시간/품질 이점", 15),
            ("1회성 스크립트 수준, 서비스화 효용 희박", 5),
            running,
        );
        running += g2;

        let g3 = ask_gate_quick(
            ui, 3, "문제의 고통과 시급성", 20,
            ("없으면 업무 마비/오류 빈번 (Must-have)", 20),
            ("있으면 편리한 수준 (Nice-to-have)", 10),
            ("단순 호기심/기술적 유희", 4),
            running,
        );
        running += g3;

        let g4 = ask_gate_quick(
            ui, 4, "유지보수 TCO", 15,
            ("표준 라이브러리 기반, 종속성 제로, 손 갈 일 없음", 15),
            ("최소한의 의존성, 가벼운 구조", 10),
            ("외부 패키지/버전 의존 높아 주기적 관리 필요", 3),
            running,
        );
        running += g4;

        let g5 = ask_gate_quick(
            ui, 5, "전략 및 자산성", 15,
            ("핵심 업무(SAS, Jarvis, 오피스 자동화) 직접 연계", 15),
            ("간접 인프라 기여", 10),
            ("주력 업무 흐름과 무관", 3),
            running,
        );

        (g1, alt, g2, g3, g4, g5)
    } else {
        // Detailed Mode
        println!("\n--- 5대 Gate 점수 직접 입력 ---");
        let g1 = prompt_number("[Gate 1] 바퀴 재발명 검증 (0~25)", 0, 25, 20);
        let alt = if g1 <= 10 {
            let a = prompt_line("  -> 대안 도구 이름 (없으면 엔터): ");
            if a.is_empty() { None } else { Some(a) }
        } else {
            None
        };
        let g2 = prompt_number("[Gate 2] 서비스화 실질 ROI (0~25)", 0, 25, 18);
        let g3 = prompt_number("[Gate 3] 문제의 고통과 시급성 (0~20)", 0, 20, 15);
        let g4 = prompt_number("[Gate 4] 유지보수 TCO (0~15)", 0, 15, 12);
        let g5 = prompt_number("[Gate 5] 전략 및 자산성 (0~15)", 0, 15, 12);
        (g1, alt, g2, g3, g4, g5)
    };

    println!("\n╭─────────────────────────────────────────────────────────────╮");
    println!("│ {} {:<47} │", ui.bold("목적 적합 아키텍처 진단"), ui.dim("(KISS & YAGNI 매핑)"));
    println!("╰─────────────────────────────────────────────────────────────╯");
    println!("구현 형태를 선택하세요:");
    println!("  1) [x] 초경량 커맨드라인 CLI (cli_tool)");
    println!("  2) [ ] Jarvis / MCP AI 에이전트 내장 도구 (agent_tool)");
    println!("  3) [ ] 단일 파일 정적 HTML 대시보드 / 시각화 (dashboard)");
    println!("  4) [ ] 고성능/대용량 단일 바이너리 CLI (high_perf_cli)");
    println!("  5) [ ] 단순 1회성 스크립트 (script)");
    let p_choice = prompt_number("선택", 1, 5, 1);
    let ptype = match p_choice {
        2 => ProjectType::AgentTool,
        3 => ProjectType::Dashboard,
        4 => ProjectType::HighPerfCli,
        5 => ProjectType::Script,
        _ => ProjectType::CliTool,
    };

    println!("\n초고속 성능/저지연이 필수적입니까?");
    println!("  1) [x] 아니오 (충분히 빠르면 됨)");
    println!("  2) [ ] 예 (대용량/극단적 성능 필요)");
    let perf_choice = prompt_number("선택", 1, 2, 1);
    let is_perf = perf_choice == 2;

    let scores = GateScores::new_clamped(g1, g2, g3, g4, g5);
    let result = evaluate(&name, scores, ptype, is_perf, alt_name.as_deref());

    // 미려한 터미널 대시보드 출력
    println!("\n{}", render_terminal_dashboard(&result, ui));

    println!("산출물 저장 옵션:");
    println!("  1) Markdown 리포트 파일로 저장 (.md)");
    println!("  2) 인터랙티브 HTML 대시보드로 저장 (.html) [브라우저 즉시 열기 가능]");
    println!("  3) 둘 다 저장 (.md + .html)");
    println!("  4) 저장하지 않고 종료");
    let save_choice = prompt_number("선택", 1, 4, 3);

    let clean_name = name.chars().map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' }).collect::<String>();

    if save_choice == 1 || save_choice == 3 {
        let default_md = format!("{}_report.md", clean_name);
        let md_path = prompt_line(&format!("Markdown 저장 경로 [기본: {}]: ", default_md));
        let path = if md_path.is_empty() { default_md } else { md_path };
        let report = format_markdown_report(&result);
        if let Err(e) = fs::write(&path, &report) {
            eprintln!("❌ Markdown 저장 실패: {}", e);
        } else {
            println!("✅ Markdown 리포트 저장 완료: {}", Path::new(&path).display());
        }
    }

    if save_choice == 2 || save_choice == 3 {
        let default_html = format!("{}_dashboard.html", clean_name);
        let html_path = prompt_line(&format!("HTML 대시보드 저장 경로 [기본: {}]: ", default_html));
        let path = if html_path.is_empty() { default_html } else { html_path };
        let html = format_html_report(&result);
        if let Err(e) = fs::write(&path, &html) {
            eprintln!("❌ HTML 대시보드 저장 실패: {}", e);
        } else {
            println!("✅ HTML 대시보드 저장 완료: {}", Path::new(&path).display());
        }
    }

    println!("\n{}", ui.dim("worth-it을 이용해 주셔서 감사합니다. 군더더기 없는 개발 되세요!"));
}

// ============================================================================
// 5. TEMPLATE GENERATOR (--init)
// ============================================================================

const DEFAULT_TEMPLATE_MD: &str = r#"# ⚖️ [프로젝트명] worth-it 가치평가 및 기획 검증서

> **작성일:** YYYY-MM-DD  
> **기획자:** kornah  
> **핵심 철학:** 군더더기 없음 · 최적화 · 간결함 · 목적 적합성 · YAGNI · 가볍고 확실하게

---

## 1. 🛑 착수 전 3대 킬러 룰 점검

- [ ] **대안 3개 조사:** 기존 오픈소스/도구 3개를 검토했는가? (대안 도구명: _____________)
- [ ] **3의 규칙:** 이 작업을 실제로 3번 이상 반복하며 비효율을 겪었는가?
- [ ] **수작업 10배 룰:** 수작업 대비 개발/유지보수 시간이 과도하게 크지 않은가?

---

## 2. 📊 5대 Gate 점수표 (총점: _____ / 100점)

| Gate | 평가 항목 | 배점 | 득점 | 근거 및 메모 |
|:---:|---|:---:|:---:|---|
| **Gate 1** | **바퀴 재발명 검증** | 25 | | 기존 대안 대비 차별점 / 사내 특화 필수 이유: |
| **Gate 2** | **서비스화 실질 ROI** | 25 | | 반복 사용 주기 / 예상 시간 절감 규모: |
| **Gate 3** | **문제의 고통/시급성** | 20 | | Must-have vs Nice-to-have 판단: |
| **Gate 4** | **유지보수 TCO** | 15 | | 의존성 규모 / 향후 관리 부담: |
| **Gate 5** | **전략 및 자산성** | 15 | | 주력 비즈니스(SAS, Jarvis, 자동화) 연계점: |

### 🚦 최종 판정 (택 1)
- [ ] **🟢 GO (80점 이상):** 정식 개발 및 서비스화 추진
- [ ] **🟡 SPIKE (60~79점):** 1일 타임박스 최소 프로토타입(PoC)만 작성
- [ ] **🔵 ADOPT (대안 우수):** 직접 개발 중단, 기성 도구/패키지 도입
- [ ] **🔴 NO-GO (60점 미만):** 착수 기각, 1회성 명령어로 대체

---

## 3. 🛠️ 목적 적합 초경량 아키텍처 처방

- **구현 형태:** (예: 단일 파일 Python CLI / Jarvis MCP 도구 / 단일 HTML / Rust 바이너리)
- **권장 기술 스택:** (표준 라이브러리 위주 작성)
- **🚫 절대 쓰지 말 것 (금지 기술):**
  - ❌ 
  - ❌ 
- **🎯 린(Lean) 구현 수칙:**
  - [ ] 파일 1~2개 이내로 끝내기
  - [ ] 외부 라이브러리 설치 최소화
  - [ ] 1초 내 실행되는 단위 테스트 1개 작성
"#;

fn handle_init(target_path: Option<&str>) {
    let path = target_path.unwrap_or("WORTH_IT.md");
    if Path::new(path).exists() {
        eprintln!("⚠️ 파일이 이미 존재합니다: {}", path);
        let ans = prompt_line("덮어쓰시겠습니까? (y/N): ");
        if ans.to_lowercase() != "y" {
            println!("취소되었습니다.");
            return;
        }
    }

    match fs::write(path, DEFAULT_TEMPLATE_MD) {
        Ok(_) => println!("✅ 기획 검증 템플릿이 생성되었습니다: {}", Path::new(path).display()),
        Err(e) => eprintln!("❌ 템플릿 생성 실패: {}", e),
    }
}

// ============================================================================
// 6. CLI ARGUMENT PARSER & DISPATCHER
// ============================================================================

fn print_help(ui: &TermUi) {
    ui.render_banner("worth-it v0.3.1 · 초경량 프로젝트 가치평가 및 아키텍처 처방기");
    println!("사용법:");
    println!("  1. 대화형 모드:      worth-it [프로젝트명]");
    println!("  2. 템플릿 생성:      worth-it --init [파일명.md]");
    println!("  3. 인자 지정 모드:    worth-it --name <이름> --scores <G1> <G2> <G3> <G4> <G5> [옵션]\n");
    println!("옵션:");
    println!("  --name <이름>        프로젝트 아이디어 이름");
    println!("  --scores G1 G2..     5대 Gate 점수 5개 (예: --scores 22 20 18 14 12)");
    println!("  --type <유형>        프로젝트 유형 (cli_tool, agent_tool, dashboard, perf, script)");
    println!("  --perf               초고속 성능 필수 여부 지정");
    println!("  --alt <대안이름>     기존에 존재하는 대안 오픈소스/도구명");
    println!("  --json               결과를 JSON 포맷으로 출력 (Jarvis/자동화 연동)");
    println!("  --html <파일.html>   결과를 독립형 다크 테마 HTML 대시보드로 저장");
    println!("  --no-color           터미널 ANSI 색상 비활성화");
    println!("  -o, --output <파일>  리포트를 Markdown 파일로 저장");
    println!("  --init [파일]        현재 폴더에 빈 WORTH_IT.md 기획 검증서 생성");
    println!("  -h, --help           도움말 출력");
}

fn main() {
    init_windows_console();

    let args: Vec<String> = env::args().collect();
    let no_color = args.iter().any(|a| a == "--no-color");
    let ui = TermUi::new(!no_color);

    if args.len() == 1 {
        run_interactive(&ui, None);
        return;
    }

    if args[1] == "--init" {
        let target = if args.len() > 2 { Some(args[2].as_str()) } else { None };
        handle_init(target);
        return;
    }

    // 단일 인자이며 옵션 플래그가 아닌 경우 프로젝트 이름으로 대화형 시작
    if args.len() == 2 && !args[1].starts_with('-') {
        run_interactive(&ui, Some(args[1].clone()));
        return;
    }

    let mut name = "unnamed-project".to_string();
    let mut scores_vec = Vec::new();
    let mut ptype = ProjectType::CliTool;
    let mut perf = false;
    let mut alt: Option<String> = None;
    let mut output: Option<String> = None;
    let mut html_output: Option<String> = None;
    let mut as_json = false;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_help(&ui);
                return;
            }
            "--name" => {
                if i + 1 < args.len() {
                    name = args[i + 1].clone();
                    i += 1;
                }
            }
            "--scores" => {
                while i + 1 < args.len() && !args[i + 1].starts_with('-') && scores_vec.len() < 5 {
                    if let Ok(v) = args[i + 1].parse::<u32>() {
                        scores_vec.push(v);
                        i += 1;
                    } else {
                        break;
                    }
                }
            }
            "--type" => {
                if i + 1 < args.len() {
                    ptype = ProjectType::parse(&args[i + 1]);
                    i += 1;
                }
            }
            "--perf" => {
                perf = true;
            }
            "--alt" => {
                if i + 1 < args.len() {
                    alt = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "--json" => {
                as_json = true;
            }
            "--html" => {
                if i + 1 < args.len() {
                    html_output = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-o" | "--output" => {
                if i + 1 < args.len() {
                    output = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {
                // 알 수 없는 플래그 무시 or 단일 위치인자 name 처리
                if !args[i].starts_with('-') && name == "unnamed-project" && scores_vec.is_empty() {
                    name = args[i].clone();
                }
            }
        }
        i += 1;
    }

    if scores_vec.len() < 5 {
        eprintln!("오류: --scores 옵션 뒤에 5개 점수(G1 G2 G3 G4 G5)를 모두 전달해야 합니다.");
        eprintln!("각 Gate 최대 배점: G1(25) G2(25) G3(20) G4(15) G5(15)");
        eprintln!("예시: worth-it --name \"my-tool\" --scores 22 20 18 14 12");
        std::process::exit(1);
    }

    // 각 Gate 점수를 최대 배점으로 안전하게 클램핑
    let scores = GateScores::new_clamped(
        scores_vec[0],
        scores_vec[1],
        scores_vec[2],
        scores_vec[3],
        scores_vec[4],
    );

    let result = evaluate(&name, scores, ptype, perf, alt.as_deref());

    // HTML 출력 플래그가 지정된 경우
    if let Some(ref h_path) = html_output {
        let html_content = format_html_report(&result);
        if let Err(e) = fs::write(h_path, &html_content) {
            eprintln!("❌ HTML 대시보드 저장 실패: {}", e);
        } else {
            println!("HTML 대시보드 저장 완료: {}", Path::new(h_path).display());
        }
    }

    if as_json {
        let json_content = format_json_report(&result);
        if let Some(out_path) = output {
            let _ = fs::write(&out_path, &json_content);
            println!("JSON 리포트 저장 완료: {}", Path::new(&out_path).display());
        } else {
            println!("{}", json_content);
        }
        return;
    }

    // Markdown 파일 저장 처리
    if let Some(ref out_path) = output {
        let report = format_markdown_report(&result);
        if let Err(e) = fs::write(out_path, &report) {
            eprintln!("❌ 파일 저장 실패: {}", e);
            std::process::exit(1);
        } else {
            println!("Markdown 리포트 저장 완료: {}", Path::new(out_path).display());
        }
    }

    // 터미널 화면에는 미려한 대시보드 카드 출력
    println!("{}", render_terminal_dashboard(&result, &ui));
}

// ============================================================================
// 7. UNIT TESTS (Zero External Crate)
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gate_scores_total() {
        let scores = GateScores::new_clamped(20, 20, 15, 15, 15);
        assert_eq!(scores.total(), 85);
    }

    #[test]
    fn test_gate_scores_clamping() {
        // 배점 초과 시 안전하게 최대치로 클램핑되는지 검증
        let scores = GateScores::new_clamped(100, 100, 100, 100, 100);
        assert_eq!(scores.gate1_alternative, 25);
        assert_eq!(scores.gate2_servitization, 25);
        assert_eq!(scores.gate3_urgency, 20);
        assert_eq!(scores.gate4_tco, 15);
        assert_eq!(scores.gate5_strategy, 15);
        assert_eq!(scores.total(), 100);
    }

    #[test]
    fn test_evaluate_go_decision() {
        let scores = GateScores::new_clamped(22, 22, 18, 14, 14);
        let res = evaluate("super-tool", scores, ProjectType::AgentTool, false, None);
        assert_eq!(res.decision, Decision::Go);
        assert_eq!(res.scores.total(), 90);
        assert!(res.prescription.forbidden_bloat[0].contains("FastAPI"));
        assert!(res.prescription.next_action.contains("custom_tools"));
    }

    #[test]
    fn test_evaluate_spike_decision() {
        let scores = GateScores::new_clamped(18, 15, 12, 12, 12);
        let res = evaluate("quick-poc", scores, ProjectType::CliTool, false, None);
        assert_eq!(res.decision, Decision::Spike);
        assert!(res.decision_reason.contains("1일 타임박스"));
    }

    #[test]
    fn test_evaluate_adopt_decision() {
        let scores = GateScores::new_clamped(4, 20, 15, 10, 10);
        let res = evaluate("custom-grep", scores, ProjectType::CliTool, false, Some("ripgrep"));
        assert_eq!(res.decision, Decision::Adopt);
        assert!(res.decision_reason.contains("ripgrep"));
        assert!(res.prescription.form_factor.contains("No-Code"));
    }

    #[test]
    fn test_evaluate_no_go_decision() {
        let scores = GateScores::new_clamped(10, 10, 10, 10, 10);
        let res = evaluate("bloat-tool", scores, ProjectType::CliTool, false, None);
        assert_eq!(res.decision, Decision::NoGo);
        assert_eq!(res.prescription.estimated_files, "0개");
    }

    #[test]
    fn test_dashboard_prescription() {
        let scores = GateScores::new_clamped(20, 20, 15, 15, 15);
        let res = evaluate("status-view", scores, ProjectType::Dashboard, false, None);
        assert!(res.prescription.form_factor.contains("단일 정적 HTML"));
        assert!(res.prescription.forbidden_bloat.iter().any(|b| b.contains("React")));
    }

    #[test]
    fn test_gauge_modern_render() {
        assert_eq!(render_gauge_modern(50, 100, 10), "▰▰▰▰▰▱▱▱▱▱");
        assert_eq!(render_gauge_modern(100, 100, 10), "▰▰▰▰▰▰▰▰▰▰");
        assert_eq!(render_gauge_modern(0, 100, 10), "▱▱▱▱▱▱▱▱▱▱");
    }

    #[test]
    fn test_escape_json_comprehensive() {
        // 제어 문자, 특수 기호, 유니코드 이스케이프 검증
        let raw = "hello \"world\"\n\r\t\x08\x0c\x01";
        let escaped = escape_json(raw);
        assert!(escaped.contains("\\\""));
        assert!(escaped.contains("\\n"));
        assert!(escaped.contains("\\r"));
        assert!(escaped.contains("\\t"));
        assert!(escaped.contains("\\b"));
        assert!(escaped.contains("\\f"));
        assert!(escaped.contains("\\u0001"));
    }

    #[test]
    fn test_escape_html() {
        let raw = "<script>alert('XSS' & \"test\");</script>";
        let escaped = escape_html(raw);
        assert_eq!(escaped, "&lt;script&gt;alert(&#x27;XSS&#x27; &amp; &quot;test&quot;);&lt;/script&gt;");
    }

    #[test]
    fn test_json_report_format() {
        let scores = GateScores::new_clamped(22, 20, 18, 14, 12);
        let res = evaluate("json-tool", scores, ProjectType::CliTool, false, None);
        let json = format_json_report(&res);
        assert!(json.contains("\"project_name\": \"json-tool\""));
        assert!(json.contains("\"decision\": \"GO\""));
        assert!(json.contains("\"total_score\": 86"));
    }

    #[test]
    fn test_html_report_xss_safety() {
        let scores = GateScores::new_clamped(22, 20, 18, 14, 12);
        let res = evaluate("<script>evil()</script>", scores, ProjectType::CliTool, false, None);
        let html = format_html_report(&res);
        assert!(html.contains("&lt;script&gt;evil()&lt;/script&gt;"));
        assert!(!html.contains("<script>evil()</script>"));
    }
}
