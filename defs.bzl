def _fluent_types_generate_impl(ctx):
  ctx.actions.run(
    inputs = ctx.files.src,
    outputs = [ctx.outputs.out],
    arguments = [ctx.files.src[0].path, "-o", ctx.outputs.out.path],
    executable = ctx.executable.executable,
  )

  return [DefaultInfo(files = depset([ctx.outputs.out]))]

fluent_types_generate = rule(
  implementation = _fluent_types_generate_impl,
  attrs = {
    "src": attr.label(
      allow_single_file = [".ftl"],
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
