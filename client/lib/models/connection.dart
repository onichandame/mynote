import 'package:json_annotation/json_annotation.dart';

part 'connection.g.dart';

abstract class Connection<TEdge extends Edge> {
  final PageInfo pageInfo;
  final List<TEdge> edges;
  Connection(this.pageInfo, this.edges);
}

@JsonSerializable()
class PageInfo {
  final bool hasPreviousPage;
  final bool hasNextPage;
  final String? startCursor;
  final String? endCursor;
  PageInfo(
      this.hasPreviousPage, this.hasNextPage, this.startCursor, this.endCursor);

  factory PageInfo.fromJson(Map<String, dynamic> json) =>
      _$PageInfoFromJson(json);
  Map<String, dynamic> toJson() => _$PageInfoToJson(this);
}

abstract class Edge<T> {
  final String cursor;
  final T node;
  Edge(this.cursor, this.node);
}
