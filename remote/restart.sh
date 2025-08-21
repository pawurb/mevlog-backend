ssh $TARGET_NODE << EOF
for session in \$(screen -ls | grep "api" | awk '{print \$1}'); do
    echo "Terminating session \$session"
    screen -X -S "\$session" quit
done

for session in \$(screen -ls | grep "cron" | awk '{print \$1}'); do
    echo "Terminating session \$session"
    screen -X -S "\$session" quit
done

EOF

ssh "$TARGET_NODE" << 'EOF'
screen -d -m -S api bash -c 'export PATH="$HOME/.foundry/bin:$HOME/.cargo/bin:$PATH" && cd /root/mevlog-backend/ && source .env && ./target/release/server > dbg.log 2>&1'
screen -d -m -S cron bash -c 'export PATH="$HOME/.foundry/bin:$HOME/.cargo/bin:$PATH" && cd /root/mevlog-backend/ && source .env && ./target/release/scheduler > dbg.log 2>&1'
EOF