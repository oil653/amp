import "package:flutter/material.dart";

class Home extends StatefulWidget {
  const Home({super.key});
  
  @override
  State<StatefulWidget> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  @override 
  Widget build(context) => Column(
    children: [
      const Text("HOME")
    ],
  );
}