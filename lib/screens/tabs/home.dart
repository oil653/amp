import "package:amp/src/bindings/signals/signals.dart";
import "package:file_picker/file_picker.dart";
import "package:flutter/material.dart";

class Home extends StatefulWidget {
  const Home({super.key});

  @override
  State<StatefulWidget> createState() => _HomeState();
}

class _HomeState extends State<Home> {
  late Playback _playback;

  void _pickFile() async {
    FilePickerResult? result = await FilePicker.platform.pickFiles(
      type: FileType.audio,
      allowMultiple: false,
    );
    if (result != null) {
      OpenMedia(
        filePath: result.paths[0]!,
        actionType: OpenMediaAction.replaceQueue,
      ).sendSignalToRust();
    }
  }

  @override
  void initState() {
    super.initState();
    _playback =
        PlaybackResponse.latestRustSignal?.message.playback ?? Playback.stopped;
    PlaybackResponse.rustSignalStream.listen((signalPack) {
      setState(() {
        _playback = signalPack.message.playback;
      });
    });
  }

  @override
  void dispose() {
    super.dispose();
  }

  @override
  Widget build(context) => Column(
    children: [
      const Text("HOME"),
      ElevatedButton(
        child: Icon(switch (_playback) {
          Playback.stopped => Icons.rectangle_outlined,
          Playback.playing => Icons.pause,
          Playback.paused => Icons.play_arrow,
        }),
        onPressed: () => _playback == Playback.playing
            ? Playback.paused.sendSignalToRust()
            : Playback.playing.sendSignalToRust(),
      ),
      StreamBuilder(
        stream: PlaybackResponse.rustSignalStream,
        builder: (context, snapshot) =>
            Text("Playback status is: ${snapshot.data?.message.playback}"),
      ),
      ElevatedButton(onPressed: _pickFile, child: const Text("Open a file")),
    ],
  );
}
