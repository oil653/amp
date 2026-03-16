import 'package:amp/screens/nav_rail.dart';
import 'package:amp/screens/tabs/home.dart';
import 'package:amp/screens/tabs/library.dart';
import 'package:amp/screens/tabs/stats.dart';
import 'package:amp/screens/tabs/zen.dart';
import 'package:rinf/rinf.dart';
import 'src/bindings/bindings.dart';
import 'package:flutter/material.dart';

// The structure of the dart project: 
// │
// ├src         Bindings for the rust backend. Automatically generated with `rinf gen`.
// ├screens     Any screen that is shown in the app. These include home, library, zen, stats, settings...
// │ └tabs        The tabs on the naviagtion bar. These include home, library, stats and zen

Future<void> main() async {
  await initializeRust(assignRustSignal);
  runApp(const MainApp());
}

enum ScreenSize {
  small,
  medium, 
  large
}

class MainApp extends StatefulWidget {
  const MainApp({super.key});

  @override
  State<StatefulWidget> createState() => _MainAppState();
}


class _MainAppState extends State<MainApp> with SingleTickerProviderStateMixin {
  // Themes
  final ThemeMode _themeMode = ThemeMode.dark;
  final ThemeData _dark = ThemeData.dark();
  final ThemeData _light = ThemeData.light();

  // Screen sizes
  static const double _mediumScreenMinWidth = 780.0;
  static const double _largeScreenMinWidth = 1280.0;

  ScreenSize _getScreenSize(double minWidth) {
    if (minWidth >= _largeScreenMinWidth) {
      return ScreenSize.large;
    } else if (minWidth >= _mediumScreenMinWidth) {
      return ScreenSize.medium;
    } else {
      return ScreenSize.small;
    }
  }


  // TABS
  final PageController _pageController = PageController();
  int _currentTabIndex = 0;

  final _tabCount = _tabWidget.length;

  void _setTab(int index) {
    if (index >= _tabCount || index < 0) {return;}
    _pageController.animateToPage(
      index, 
      duration: const Duration(milliseconds: 300), 
      curve: Curves.ease
    );
  }

  static const List<NavigationDestination> _tabDestinations = <NavigationDestination> [
    NavigationDestination(icon: Icon(Icons.home), label: "Home"),
    NavigationDestination(icon: Icon(Icons.self_improvement), label: "Zen"),
    NavigationDestination(icon: Icon(Icons.library_music), label: "Library"),
    NavigationDestination(icon: Icon(Icons.stacked_line_chart), label: "Stonks")
  ];
  static const List<NavigationRailDestination> _tabRailDestinations = <NavigationRailDestination> [
    NavigationRailDestination(icon: Icon(Icons.home_outlined), selectedIcon: Icon(Icons.home), label: Text("Home")),
    NavigationRailDestination(icon: Icon(Icons.self_improvement), label: Text("Zen")),
    NavigationRailDestination(icon: Icon(Icons.library_music), label: Text("Library")),
    NavigationRailDestination(icon: Icon(Icons.stacked_line_chart), label: Text("Stonks"))
  ];

  /// The widgets of each tab
  static const List<Widget> _tabWidget = <Widget>[
    Home(),
    Zen(),
    Library(),
    Stats()
  ];

  @override
  Widget build(BuildContext context) => LayoutBuilder(
    builder: (context, constrains) {
      final ScreenSize screenSize = _getScreenSize(constrains.minWidth);

      return MaterialApp(
        themeMode: _themeMode,
        theme: _light,
        darkTheme: _dark,

        home: Scaffold(
          bottomNavigationBar: screenSize == ScreenSize.small ? NavigationBar(
            destinations: _tabDestinations,
            onDestinationSelected: _setTab,
            selectedIndex: _currentTabIndex,
          ) : null,

          body: _buildBody(screenSize)
        )
      );
    }
  );

  Widget _buildBody(ScreenSize screenSize) => SafeArea(
    child: Row(
      children: [
        // Left navbar, only available on not small screens
        if (screenSize != ScreenSize.small) Navrail(
          selectedIndex: _currentTabIndex,
          destinations: _tabRailDestinations,
          onDestinationSelected: (idx) => _setTab(idx),

          labelType: screenSize == ScreenSize.medium ? NavigationRailLabelType.selected : null,
          extended: screenSize == ScreenSize.large ? true : false,
          minExtendedWidth: 150,
        ),

        // Main content
        Expanded(
          child: PageView(
            controller: _pageController,
            onPageChanged: (idx) => setState(() => _currentTabIndex = idx),
            children: _tabWidget
          )
        )
      ],
    )
  );
}