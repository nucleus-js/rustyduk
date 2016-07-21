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

print("envkeys: ", nucleus.envkeys())

print("envkeys(true): ", nucleus.envkeys(true))

print("exit: ", nucleus.exit)

nucleus.exit(0);
print("Error: do not print me!")
