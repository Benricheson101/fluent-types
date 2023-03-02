def _fluent_types_generate_impl(ctx):
  files = [f.path for f in ctx.files.srcs]

  ctx.actions.run(
    inputs = ctx.files.srcs,
    outputs = [ctx.outputs.out],
    arguments = files + ["-o", ctx.outputs.out.path],
    executable = ctx.executable.executable,
  )

  return [DefaultInfo(files = depset([ctx.outputs.out]))]

fluent_types_generate = rule(
  implementation = _fluent_types_generate_impl,
  attrs = {
    "srcs": attr.label_list(
      allow_files = [".ftl"],
      mandatory = True,
    ),

    "out": attr.output(
      mandatory = True,
    ),

    "executable": attr.label(
      executable = True,
      default = Label("//:fluent-types"),
      cfg = "exec",
    ),
  }
)
