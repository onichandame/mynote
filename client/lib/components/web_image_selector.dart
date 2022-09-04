import 'dart:async';

import 'package:flutter/material.dart';

class WebImageSelector extends StatefulWidget {
  final Widget? title;
  final FutureOr Function(String value) onSubmit;
  const WebImageSelector({Key? key, required this.onSubmit, this.title})
      : super(key: key);

  @override
  State<WebImageSelector> createState() => _WebImageSelectorState();
}

class _WebImageSelectorState extends State<WebImageSelector> {
  String? _url;
  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: widget.title,
      content: Wrap(
        crossAxisAlignment: WrapCrossAlignment.center,
        alignment: WrapAlignment.center,
        children: [
          ConstrainedBox(
            constraints: const BoxConstraints(
                maxWidth: 255, minWidth: 255, maxHeight: 255, minHeight: 255),
            child: _url == null
                ? Container()
                : Image.network(_url!,
                    errorBuilder: (_, __, ___) =>
                        const Text('image url not valid')),
          ),
          TextFormField(
            decoration: InputDecoration(label: widget.title),
            onChanged: (value) {
              setState(() {
                _url = value;
              });
            },
            onFieldSubmitted: (_) async {
              await widget.onSubmit(_url!);
              if (mounted) Navigator.of(context).pop();
            },
          )
        ],
      ),
    );
  }
}
