# GEMINI.md

## 프로젝트 개요
- 설명: Tailscale 네트워크를 통해 연결된 원격 Windows 서버(경로: gram)와 local mac 서버의 CPU 사용량을 실시간으로 모니터링하고, 사용률에 비례하여 속도가 변하는 애니메이션을 터미널과 메뉴 막대에 출력하는 프로젝트입니다.
- 주요 아키텍처:
    - SSH Data Stream: Mac(Local)에서 gram(Windows)으로 SSH 접속 후 PowerShell 명령어로 데이터 추출.
    - Dynamic Frame Controller: 수신된 데이터(0-100%)를 애니메이션 딜레이 값으로 매핑.
    - CLI Renderer: crossterm을 이용한 터미널 버퍼 제어.

## 기술적 제약 사항 및 보안 규칙
- 언어: Rust (Edition 2024)
- 네트워크 및 보안 가이드라인:
    - Connectivity: Tailscale망 내 gram 호스트에 연결합니다.
    - Passwordless: Mac의 SSH Key가 gram의 authorized_keys에 등록되어 있어야 합니다.
    - Safe Memory: unsafe 블록 사용을 지양하고 Rust의 소유권 모델을 적극 활용합니다.
- 성능: Windows의 PowerShell 쿼리 부하를 고려하여 데이터 수집 간격을 최적화합니다.

## 코딩 스타일
- Naming: 모든 변수와 함수는 snake_case를 사용합니다.
- Modularity: SSH 통신 로직(ssh.rs), 데이터 파싱(parser.rs), 애니메이션 렌더링(display.rs)으로 분리합니다.

## 자주 쓰는 명령어
- 빌드: cargo build
- 실행: cargo run

## 참고 및 리소스
- 데이터 소스: Windows PowerShell (Get-Counter)
- 대상 호스트: gram (Windows 11 / Tailscale)