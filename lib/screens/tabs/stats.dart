import "package:flutter/material.dart";

class Stats extends StatefulWidget {
  const Stats({super.key});
  
  @override
  State<StatefulWidget> createState() => _StatsState();
}

class _StatsState extends State<Stats> {
  @override 
  Widget build(context) => Column(
    children: [
      const Text("STATS")
    ],
  );
}