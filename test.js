print("Hello World!")

print("nucleus: ", nucleus)

print("cmd: ", nucleus.cmd)

print("rawArgs: ", nucleus.rawArgs)

for (var i = 0; i < nucleus.rawArgs.length; i++) {
  print("rawArgs[" + i + "]: ", nucleus.rawArgs[i])
}

print("engine: ", nucleus.engine)

print("versions: ", nucleus.versions)

print("versions.duktape: ", nucleus.versions.duktape)
