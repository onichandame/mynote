import 'package:flutter/material.dart';
import 'package:notebook/components/layout.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/dashboard/note/main.dart';
import 'package:notebook/screens/dashboard/todo.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class DashboardScreen extends StatefulWidget {
  const DashboardScreen({Key? key}) : super(key: key);

  @override
  State<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends State<DashboardScreen> {
  int _currentIndex = 1;
  @override
  Widget build(BuildContext context) {
    return Consumer<CurrentUser?>(builder: (builder, currentUser, child) {
      final loggedIn = currentUser?.user != null;
      Widget getBody() {
        switch (_currentIndex) {
          case 0:
            return const TodoList();
          case 1:
            return const NoteMain();
          default:
            throw UnimplementedError('This screen is not implemented yet');
        }
      }

      return Layout(
          title: 'Notebook',
          body: loggedIn ? getBody() : const _Guest(),
          bottomNavigationBar: BottomNavigationBar(
            currentIndex: _currentIndex,
            onTap: (value) {
              setState(() {
                _currentIndex = value;
              });
            },
            items: const [
              BottomNavigationBarItem(
                icon: Icon(Icons.list_sharp),
                label: 'Todo',
              ),
              BottomNavigationBarItem(
                  icon: Icon(Icons.note_sharp), label: 'Note'),
              BottomNavigationBarItem(
                  icon: Icon(Icons.report_sharp), label: 'Report')
            ],
          ));
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
