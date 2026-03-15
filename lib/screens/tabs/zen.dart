import "package:flutter/material.dart";

class Zen extends StatefulWidget {
  const Zen({super.key});
  
  @override
  State<StatefulWidget> createState() => _ZenState();
}

class _ZenState extends State<Zen> {
  @override 
  Widget build(context) => Column(
    children: [
      const Text("ZEN")
    ],
  );
}