# Usage:
#   make run                          # 最新の問題を実行
#   make run contest=abc/445 problem=a  # abc445-a の最新版を実行
#   make new contest=abc/445 problem=c  # 新しい問題を作成

run:
ifdef contest
	bash scripts/run.sh $(contest) $(problem)
else
	bash scripts/run.sh
endif

new:
	bash scripts/new.sh $(contest) $(problem)
