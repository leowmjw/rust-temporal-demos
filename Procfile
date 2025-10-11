temporal: ~/go/bin/temporal server start-dev --port 7233 --ui-port 8233 --metrics-port 57271
# api: pkgx air .
worker: cd food-ordering/worker && pkgx air .
kickoff: cd food-ordering/starter && pkgx air .
web: cd food-ordering/web && npm run dev

