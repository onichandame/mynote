import 'package:flutter/material.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/signup.dart';
import 'package:notebook/screens/loading.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class HomeScreen extends StatelessWidget {
  static const routeName = routeHome;

  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final client = Provider.of<Client?>(context);
    if (client == null) {
      Future.microtask(() {
        Navigator.pushNamed(context, LoadingScreen.routeName);
      });
    }
    return Navigator(
      initialRoute: routeHome,
      onGenerateRoute: (settings) {
        late Widget page;
        if (settings.name == routeSignup) {
          page = const SignupScreen();
        } else if (settings.name == routeHome) {
          page = Scaffold(
              appBar: AppBar(
                title: const Text('Notebook'),
                actions: const [Avatar()],
              ),
              body: const Center(
                child: _Home(),
              ));
        } else {
          throw Exception('404: Unknown route ${settings.name}');
        }
        return MaterialPageRoute(
            builder: (context) => page, settings: settings);
      },
    );
  }
}

class _Home extends StatelessWidget {
  const _Home({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<CurrentUser?>(
        builder: (builder, currentUser, child) =>
            currentUser?.user == null ? const _Guest() : const _Dashboard());
  }
}

class _Guest extends StatelessWidget {
  const _Guest({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          const Text("Welcom to your private notebook"),
          Row(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              TextButton(
                  onPressed: () {
                    Navigator.pushNamed(context, routeSignup);
                  },
                  child: const Text("signup")),
              TextButton(
                  onPressed: () {
                    Navigator.pushNamed(context, routeLogin);
                  },
                  child: const Text("login"))
            ],
          )
        ],
      ),
    );
  }
}

class _Dashboard extends StatelessWidget {
  const _Dashboard({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return const Text('TODO: dashboard here');
  }
}
