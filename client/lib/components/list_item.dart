import 'package:flutter/material.dart';

class ListItem extends StatelessWidget {
  final Widget title;
  final Widget? leading;
  final Function() onTap;
  final Widget? value;

  const ListItem(
      {Key? key,
      required this.title,
      this.leading,
      required this.onTap,
      this.value})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListTile(
      leading: leading,
      title: title,
      trailing: Wrap(
        crossAxisAlignment: WrapCrossAlignment.center,
        spacing: 10,
        children: [
          if (value != null) value!,
          const Icon(Icons.arrow_forward_ios)
        ],
      ),
      onTap: onTap,
    );
  }
}
