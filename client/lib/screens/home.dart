import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class HomeScreen extends StatelessWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer<CurrentUser?>(builder: (builder, currentUser, child) {
      final loggedIn = currentUser?.user != null;
      return Layout(
        title: 'Notebook',
        body: loggedIn ? const _Dashboard() : const _Guest(),
        bottomNavigationBar: loggedIn
            ? BottomNavigationBar(
                items: [
                  BottomNavigationBarItem(
                      icon: Icon(Icons.list_sharp), label: 'Todo'),
                  BottomNavigationBarItem(
                      icon: Icon(Icons.bookmark_sharp), label: 'Log'),
                  BottomNavigationBarItem(
                      icon: Icon(Icons.report_sharp), label: 'Report')
                ],
              )
            : null,
      );
    });
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
