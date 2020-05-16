;; Eval Buffer with `M-x eval-buffer' to register the newly created template.
;; "/Users/motchang/.cargo/bin/rust-lldb"

(dap-register-debug-template "Debug executable 'rpn'"
			     (list :type "lldb"
				   :request "launch"
				   :cargo (list :args ["build" "--bin=rpn" "--package=apn"]
						:filter (list :name "rpn"
							      :kind "bin"))
				   :args []
				   :cwd "/Users/motchang/src/github.com/motchang/rust-bicycle-book/rpn/"
				   :
				   :terminal "console"
				   :sourceLanguages ["rust"]))

(dap-register-debug-template "Debug executable with lldb-vscode"
			     (list :type "lldb"
				   :request "launch"
				   :program "target/debug/rpn"
				   :args []
				   :cwd "/Users/motchang/src/github.com/motchang/rust-bicycle-book/rpn/"
				   :debuggerRoot "/Users/motchang/src/github.com/motchang/rust-bicycle-book/rpn/"
				   :terminal "console"
				   :sourceLanguages ["rust"]))

















(dap-register-debug-template "Debug unit tests in executable 'rpn'"
			     (list :type "lldb"
				   :request "launch"
				   :cargo (list :args ["test" "--no-run" "--bin=rpn" "--package=apn"]
						:filter (list :name "rpn"
							      :kind "bin"))
				   :args []
				   :cwd "/Users/motchang/src/github.com/motchang/rust-bicycle-book/rpn/"))
