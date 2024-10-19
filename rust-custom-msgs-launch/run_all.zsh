#!/usr/bin/env zsh

# 현재 tmux 세션 이름 가져오기
SESSION_NAME=$(tmux display-message -p '#S')

# 현재 패널 번호 가져오기
CURRENT_PANE=$(tmux display-message -p '#P')

# 서브스크라이버 실행 (현재 패널)
tmux send-keys -t ${SESSION_NAME}.${CURRENT_PANE} 'source ../../install/setup.zsh' C-m
sleep 1
tmux send-keys -t ${SESSION_NAME}.${CURRENT_PANE} 'cargo run --bin subscriber' C-m
sleep 1

# 현재 윈도우를 가로로 분할
tmux split-window -h
sleep 1

# 새로운 패널 번호 가져오기 (분할 후 오른쪽 패널)
NEW_PANE=$(tmux display-message -p '#P')

# 퍼블리셔 실행 (새로 생성된 패널)
tmux send-keys -t ${SESSION_NAME}.${NEW_PANE} 'source ../../install/setup.zsh' C-m
sleep 1
tmux send-keys -t ${SESSION_NAME}.${NEW_PANE} 'cargo run --bin publisher' C-m

# 첫 번째 패널로 포커스 이동 (서브스크라이버가 실행된 패널)
tmux select-pane -t ${SESSION_NAME}.${CURRENT_PANE}
