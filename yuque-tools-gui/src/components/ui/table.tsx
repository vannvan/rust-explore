import React from 'react'

interface TableProps {
  children: React.ReactNode
  className?: string
}

interface TableHeaderProps {
  children: React.ReactNode
  className?: string
}

interface TableBodyProps {
  children: React.ReactNode
  className?: string
}

interface TableRowProps {
  children: React.ReactNode
  className?: string
}

interface TableHeadProps {
  children: React.ReactNode
  className?: string
}

interface TableCellProps {
  children: React.ReactNode
  className?: string
  colSpan?: number
}

const Table: React.FC<TableProps> = ({ children, className = '' }) => {
  return (
    <div className={`w-full overflow-auto ${className}`}>
      <table className="w-full caption-bottom text-sm">{children}</table>
    </div>
  )
}

const TableHeader: React.FC<TableHeaderProps> = ({ children, className = '' }) => {
  return <thead className={`border-b border-gray-200 bg-gray-50 ${className}`}>{children}</thead>
}

const TableBody: React.FC<TableBodyProps> = ({ children, className = '' }) => {
  return <tbody className={`divide-y divide-gray-200 ${className}`}>{children}</tbody>
}

const TableRow: React.FC<TableRowProps> = ({ children, className = '' }) => {
  return <tr className={`hover:bg-gray-50 transition-colors ${className}`}>{children}</tr>
}

const TableHead: React.FC<TableHeadProps> = ({ children, className = '' }) => {
  return (
    <th
      className={`px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider ${className}`}
    >
      {children}
    </th>
  )
}

const TableCell: React.FC<TableCellProps> = ({ children, className = '', colSpan }) => {
  return (
    <td
      className={`px-6 py-4 whitespace-nowrap text-sm text-gray-900 ${className}`}
      colSpan={colSpan}
    >
      {children}
    </td>
  )
}

export { Table, TableHeader, TableBody, TableRow, TableHead, TableCell }
