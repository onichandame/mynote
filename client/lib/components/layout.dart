import 'package:flutter/material.dart';
import 'package:notebook/components/avatar.dart';
import 'package:notebook/components/list_item.dart';
import 'package:notebook/providers/client.dart';
import 'package:notebook/providers/current_user.dart';
import 'package:notebook/screens/routes.dart';
import 'package:provider/provider.dart';

class Layout extends StatelessWidget {
  final Widget body;
  final String title;
  final Widget? bottomNavigationBar;

  const Layout(
      {Key? key,
      required this.title,
      required this.body,
      this.bottomNavigationBar})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        leading: Navigator.canPop(context)
            ? IconButton(
                icon: const Icon(Icons.arrow_back),
                onPressed: () {
                  Navigator.pop(context);
                },
              )
            : null,
        title: Text(title),
        actions: [
          Avatar(
            onPressed: (context) {
              Scaffold.of(context).openEndDrawer();
            },
          )
        ],
      ),
      endDrawer: const AvatarDrawer(),
      bottomNavigationBar: bottomNavigationBar,
      body: body,
    );
  }
}

class AvatarDrawer extends StatelessWidget {
  const AvatarDrawer({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Consumer2<CurrentUser?, Client?>(
        builder: (context, currentUser, client, _) => Drawer(
              child: ListView(children: [
                ...(currentUser?.user == null
                    ? [
                        DrawerItem(
                          icon: Icons.login,
                          title: 'Log In',
                          onTap: () =>
                              _navigateFromDrawerItem(context, routeLogin),
                        ),
                        DrawerItem(
                          icon: Icons.account_circle,
                          title: 'Sign Up',
                          onTap: () =>
                              _navigateFromDrawerItem(context, routeSignup),
                        ),
                      ]
                    : [
                        DrawerItem(
                            title: 'Log Out',
                            color: Theme.of(context).colorScheme.error,
                            icon: Icons.logout,
                            onTap: () {
                              client!.session = null;
                              Navigator.of(context).pushNamedAndRemoveUntil(
                                  routeHome, (_) => false);
                            }),
                      ]),
                const Divider(),
                DrawerItem(
                  icon: Icons.settings,
                  title: 'Settings',
                  onTap: () => _navigateFromDrawerItem(context, routeSettings),
                ),
              ]),
            ));
  }
}

class DrawerItem extends StatelessWidget {
  final String title;
  final IconData icon;
  final Color? color;
  final Function() onTap;

  const DrawerItem(
      {Key? key,
      required this.title,
      this.color,
      required this.onTap,
      required this.icon})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListItem(
      leading: Icon(
        icon,
        color: color,
      ),
      title: Text(
        title,
        style: color == null ? null : TextStyle(color: color),
      ),
      onTap: onTap,
    );
  }
}

void _navigateFromDrawerItem(BuildContext context, String target) {
  Navigator.pop(context);
  Navigator.pushNamed(context, target);
}
