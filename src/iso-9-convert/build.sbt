lazy val root = (project in file("."))
  .settings(
    name := "iso-9-convert",
    version := "0.1.0",
    libraryDependencies += "com.github.scopt" %% "scopt" % "3.5.0",
    scalaVersion := "2.11.8"
  )
