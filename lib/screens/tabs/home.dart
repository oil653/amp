import "package:amp/src/bindings/signals/signals.dart";
import "package:flutter/material.dart";

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<StatefulWidget> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  late TextEditingController _editingController;

  @override
  void initState() {
    super.initState();
    _editingController = TextEditingController();
  }

  @override
  void dispose() {
    _editingController.dispose();
    super.dispose();
  }

  @override
  Widget build(context) => Column(
    children: [
      const Text("HOME"),
      ElevatedButton(
        child: const Text("Lick me twin (^> - <^)"),
        onPressed: () => Playback.playing.sendSignalToRust(),
      ),
      StreamBuilder(
        stream: PlaybackResponse.rustSignalStream,
        builder: (context, snapshot) =>
            Text("Playback status is: ${snapshot.data?.message.playback}"),
      ),
      TextField(
        controller: _editingController,
        onSubmitted: (input) => OpenMedia(
          filePath: input,
          actionType: OpenMediaAction.replaceQueue,
        ).sendSignalToRust(),
      ),
    ],
  );
}
