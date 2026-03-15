import "package:flutter/material.dart";

class Library extends StatefulWidget {
  const Library({super.key});
  
  @override
  State<StatefulWidget> createState() => _LibraryState();
}

class _LibraryState extends State<Library> {
  @override 
  Widget build(context) => Column(
    children: [
      const Text("LIBRARY")
    ],
  );
}