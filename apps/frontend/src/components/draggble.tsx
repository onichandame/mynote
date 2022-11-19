import { Identifier } from "dnd-core"
import { PropsWithChildren, useRef } from "react"
import { useDrag, useDrop } from "react-dnd"

export function Draggable<Item>({
  item,
  onHover,
  onDrop,
  type,
  children,
}: Props<Item>) {
  const ref = useRef<HTMLDivElement>(null)
  const [{ isDragging }, drag] = useDrag(() => ({
    type,
    collect: monitor => ({ isDragging: !!monitor.isDragging() }),
    item: () => ({ item }),
  }))
  const [{ handlerId }, drop] = useDrop<
    { item: Item },
    void,
    { handlerId: Identifier | null }
  >({
    accept: type,
    collect: monitor => ({ handlerId: monitor.getHandlerId() }),
    hover: src => {
      if (!ref.current) return
      const source = src.item
      const target = item
      onHover(source, target)
    },
    drop: () => {
      onDrop()
    },
  })
  drag(drop(ref))
  return (
    <div
      ref={ref}
      style={{ opacity: isDragging ? 0.5 : 1, cursor: `move` }}
      data-handler-id={handlerId}
    >
      {children}
    </div>
  )
}

type Props<Item> = PropsWithChildren & {
  onHover: (source: Item, target: Item) => void
  onDrop: () => void
  item: Item
  type: DraggableItemType
}

export enum DraggableItemType {
  Memo = `Memo`,
}
